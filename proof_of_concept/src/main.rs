#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;

use core::fmt::Write;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, ExceptionFrame};
use cortex_m_semihosting::{hprintln};
use cortex_m_semihosting::hio::{self};

#[entry]
fn main() -> ! {
    hprintln!("Hello, World!").unwrap();

    let p = cortex_m::Peripherals::take().unwrap();

    let mut syst = p.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000);
    syst.enable_interrupt();
    syst.enable_counter();

    loop {
    }
}

#[exception]
fn SysTick() {
    let mut stdout = hio::hstdout().unwrap();
    write!(stdout, ".").unwrap();
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
