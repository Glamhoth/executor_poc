#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

extern crate atsamx7x_hal as hal;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;

use core::fmt::Write;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hio::{self};
use hal::target_device;

use hal::ehal::watchdog::WatchdogDisable;

#[entry]
fn main() -> ! {
    let peripherals = target_device::Peripherals::take().unwrap();
    let wdt = peripherals.WDT;
    hal::watchdog::Watchdog::new(wdt).disable();

    let mut stdout = hio::hstdout().unwrap();
    write!(stdout, "Hello, World!").unwrap();

    let p = cortex_m::Peripherals::take().unwrap();

    let mut syst = p.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000);
    syst.enable_interrupt();
    syst.enable_counter();

    loop {}
}

#[exception]
fn SysTick() {
    let mut stdout = hio::hstdout().unwrap();
    write!(stdout, ".").unwrap();
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
