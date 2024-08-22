// #![deny(warnings)]
#![no_main]
#![no_std]

#[cfg(feature = "defmt")]
use defmt_rtt as _; // global logger
use panic_probe as _;

use stm32h7xx_hal::{pac, prelude::*};

use rustBoot_hal::{stm::stm32h723::FlashWriterEraser, FlashInterface};
use rustBoot_update::update::{update_flash::FlashUpdater, UpdateInterface};

use cortex_m_rt::entry;

use core::cell::RefCell;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    //GPIO init
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // Configure PB0 as output.
    let mut led1 = gpiob.pb0.into_push_pull_output();
    // Configure PB14 as output.
    let mut led2 = gpiob.pb14.into_push_pull_output();
    let mut led_yellow = gpioe.pe1.into_push_pull_output();

    led_yellow.set_low(); /* Off */

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    let flsh = dp.FLASH;

    let mut count = 0;

    while count < 4 {
        led2.set_high();
        delay.delay_ms(500_u16);
        led2.set_low();
        delay.delay_ms(500_u16);
        count = count + 1;
    }

    let flash_writer = FlashWriterEraser { nvm: flsh, yellow_led: RefCell::new(led_yellow) };
    let updater = FlashUpdater::new(flash_writer);

    match updater.update_success() {
        Ok(_v) => { led1.set_high(); /* Turn green LED solid on when success is notified to rustboot */ }
        Err(e) => panic!("couldnt trigger update: {}", e),
    }

    loop {
        led2.toggle();
        delay.delay_ms(500_u16);
    }
}
