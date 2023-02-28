#![no_std]
#![no_main]
// #![allow(warnings, unused)]

extern crate atsamx7x_hal as hal;
extern crate panic_semihosting;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::debug;

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();

    let mut syst = peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(800_000);
    syst.enable_interrupt();
    syst.enable_counter();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[exception]
fn SysTick() {
}
