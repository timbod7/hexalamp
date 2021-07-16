#![no_main]
#![no_std]

//use panic_rtt_target as _;
//use rtt_target::rtt_init_default;

#![no_main]

// you can put a breakpoint on `rust_begin_unwind` to catch panics
use panic_halt as _;
use ws2812_spi as ws2812;
use stm32f1xx_hal as hal;

use rtic::app;
use rtic::cyccnt::{Duration};

use cortex_m::peripheral::DWT;

use stm32f1xx_hal::{
    gpio::{gpiob, gpioc::PC13, Output, PushPull, State, Alternate, Input, Floating},
    pac,
    prelude::*,
    timer::{CountDownTimer, Event, Timer},
};
use crate::hal::spi::Spi;

use crate::ws2812::Ws2812;

use smart_leds::{SmartLedsWrite};

mod animation;
use animation::{Animation};


type Display = ws2812_spi::Ws2812<Spi<stm32f1xx_hal::pac::SPI2, hal::spi::Spi2NoRemap, (gpiob::PB13<Alternate<PushPull>>, gpiob::PB14<Input<Floating>>, gpiob::PB15<Alternate<PushPull>>), u8>>;

#[app(device = stm32f1xx_hal::pac, monotonic = rtic::cyccnt::CYCCNT, peripherals = true)]
const APP: () = {
    struct Resources {
        led: PC13<Output<PushPull>>,
        timer_handler: CountDownTimer<pac::TIM1>,
        display: Display,
    }

    #[init(schedule = [animate])]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();
        // required on Cortex-M7 devices that software lock the DWT (e.g. STM32F7)
        DWT::unlock();
        cx.core.DWT.enable_cycle_counter();

        // Take ownership over the raw flash and rcc devices and convert them into the corresponding
        // HAL structs
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();

        // Freeze the configuration of all the clocks in the system and store the frozen frequencies
        // in `clocks`
        
        let clocks = rcc
          .cfgr
          .sysclk(48.mhz())
          .pclk1(24.mhz())
          .freeze(&mut flash.acr);

        // Acquire the GPIOC peripheral
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.apb2);
        let mut gpioc = cx.device.GPIOC.split(&mut rcc.apb2);

        // Configure gpio C pin 13 (LED) as a push-pull output for debugging
        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, State::High);

        // Configure SPI for ws2812 leds
        let spi_pins = (
          gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh),
          gpiob.pb14.into_floating_input(&mut gpiob.crh),
          gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh),
        );
        let spi = Spi::spi2(cx.device.SPI2, spi_pins, ws2812::MODE, 3.mhz(), clocks, &mut rcc.apb1);
        let display: Display = Ws2812::new(spi);

        // Configure the syst timer to trigger an update every second and enables interrupt
        let mut timer =
            Timer::tim1(cx.device.TIM1, &clocks, &mut rcc.apb2).start_count_down(100.hz());
        timer.listen(Event::Update);

        cx.schedule.animate(cx.start).unwrap();

        // Init the static resources to use them later through RTIC
        init::LateResources {
            led,
            timer_handler: timer,
            display
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
      loop {
        cortex_m::asm::wfi();
      }
    }

    #[task(schedule = [animate], priority = 2, resources = [display])]
    fn animate(cx: animate::Context) {
      static mut ANIM: Option<AnimType> = None;
      static mut FRAME: Option<animation::Frame>  = None;

      let anim = ANIM.get_or_insert_with(|| {
        AnimType::new()
      });
      let mut frame = FRAME.get_or_insert_with(|| {
        animation::initFrame()
      });

      cx.resources.display.write(frame.iter().cloned()).unwrap();
      let delayms = anim.next_frame(&mut frame);
      cx.resources.display.write(frame.iter().cloned()).unwrap();
      let delay_cycles = Duration::from_cycles(delayms as u32 * 48_000u32);
      cx.schedule.animate(cx.scheduled + delay_cycles).unwrap();
    }

    #[task(binds = TIM1_UP, priority = 1, resources = [led, timer_handler])]
    fn tick(cx: tick::Context) {
        cx.resources.led.toggle().unwrap();
        // Clears the update flag
        cx.resources.timer_handler.clear_update_interrupt_flag();
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
      fn USART1();
  }
};

type AnimType = animation::combo1::Anim;
