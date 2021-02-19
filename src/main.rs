#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_default;

use stm32f1xx_hal as hal;
use ws2812_spi as ws2812;

use crate::hal::delay::Delay;
use crate::hal::pac;
use crate::hal::prelude::*;
use crate::hal::spi::Spi;
use crate::ws2812::Ws2812;
use cortex_m::peripheral::Peripherals;

use smart_leds::{SmartLedsWrite};

use cortex_m_rt::entry;

mod animation;
use animation::{Animation};

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

        // Acquire the GPIOA peripheral
        let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

        let pins = (
            gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh),
            gpiob.pb14.into_floating_input(&mut gpiob.crh),
            gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh),
        );
        let mut delay = Delay::new(cp.SYST, clocks);

        let spi = Spi::spi2(dp.SPI2, pins, ws2812::MODE, 3.mhz(), clocks, &mut rcc.apb1);

        let mut ws = Ws2812::new(spi);

        
        let mut anim = animation::Blinky::new();
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
