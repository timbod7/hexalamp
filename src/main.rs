#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_default;

use cortex_m_rt::{entry};
use cortex_m::{interrupt::Mutex};

use stm32f1xx_hal as hal;
use ws2812_spi as ws2812;

use crate::hal::delay::Delay;
use crate::hal::pac;
use crate::hal::gpio;
use crate::hal::prelude::*;
use crate::hal::spi::Spi;
use crate::hal::device::interrupt;
use crate::hal::timer::{Event, Timer, CountDownTimer};
use embedded_hal::digital::v2::OutputPin;

use crate::ws2812::Ws2812;
use cortex_m::peripheral::Peripherals;

use core::cell::RefCell;


use debounced_pin::prelude::*;
use debounced_pin::ActiveHigh;

use smart_leds::{SmartLedsWrite};


mod animation;
use animation::{Animation};

// Make IO globally available
type DEBUGLEDPIN = gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>;
static G_LED: Mutex<RefCell<Option<DEBUGLEDPIN>>> = Mutex::new(RefCell::new(None));
static G_TIM: Mutex<RefCell<Option<CountDownTimer<pac::TIM2>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_default!();

    if let (Some(dp), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        // Take ownership over the raw flash and rcc devices and convert them into the corresponding
        // HAL structs
        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();

        // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
        // `clocks`
        let clocks = rcc
            .cfgr
            .sysclk(48.mhz())
            .pclk1(24.mhz())
            .freeze(&mut flash.acr);

        // Acquire the GPIOS peripherals
        let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
        let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

        // Configure the debug LED
        let debug_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        cortex_m::interrupt::free(|cs| *G_LED.borrow(cs).borrow_mut() = Some(debug_led));

        // Configure SPI for ws2812 leds
        let spi_pins = (
            gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh),
            gpiob.pb14.into_floating_input(&mut gpiob.crh),
            gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh),
        );
        let spi = Spi::spi2(dp.SPI2, spi_pins, ws2812::MODE, 3.mhz(), clocks, &mut rcc.apb1);
        let mut ws = Ws2812::new(spi);

        // Configure buttons
        let b0 = gpiob.pb10.into_pull_down_input(&mut gpiob.crh);
        // let b1 = gpiob.pb11.into_pull_down_input(&mut gpiob.crh);
        // let b2 = gpiob.pb12.into_pull_down_input(&mut gpiob.crh);

        // Setup interrupt driven timer, and move to shared variable
        let mut timer = Timer::tim2(dp.TIM2, &clocks, &mut rcc.apb1).start_count_down(10.hz());
        timer.listen(Event::Update);
        cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));
        unsafe {
          cortex_m::peripheral::NVIC::unmask(pac::Interrupt::TIM2);
        }

        let bdb0 = DebouncedInputPin::new(b0, ActiveHigh);

        let mut delay = Delay::new(cp.SYST, clocks);

        let mut anim = animation::anim4::Anim::new();
        let mut frame = anim.init_frame();
        ws.write(frame.iter().cloned()).unwrap();

        loop {
          let delayms = anim.next_frame(&mut frame);
          ws.write(frame.iter().cloned()).unwrap();
          delay.delay_ms(delayms);
        }
    }
    loop {
        continue;
    }
}

#[interrupt]
fn TIM2() {
  static mut TIM: Option<CountDownTimer<pac::TIM2>> = None;
  static mut LED: Option<DEBUGLEDPIN> = None;

  let led = LED.get_or_insert_with(|| {
    cortex_m::interrupt::free(|cs| {
        // Move LED pin here, leaving a None in its place
        G_LED.borrow(cs).replace(None).unwrap()
    })
  });

  let tim = TIM.get_or_insert_with(|| {
    cortex_m::interrupt::free(|cs| {
        G_TIM.borrow(cs).replace(None).unwrap()
    })
  });

  let _ = tim.wait();
  let _ = led.toggle();
}
