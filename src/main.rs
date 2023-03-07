#![no_std]
#![no_main]
#![allow(warnings, unused)]
#![feature(const_trait_impl)]

mod rtos;

extern crate atsamx7x_hal as hal;
extern crate panic_semihosting;

use core::ffi::c_void;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use crate::rtos::executor::Executor;
use crate::rtos::task::TaskHandle;
use crate::rtos::tasklet::Tasklet;

fn task_a(data: u32) {
    hprintln!("{}", data);
}

fn task_b(data: u8) {
    hprintln!("{}", data);
}

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();

    let mut syst = peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(800_000);
    syst.enable_interrupt();
    syst.enable_counter();

    static taska: Tasklet<u32> = Tasklet::new(task_a);
    static taskb: Tasklet<u8> = Tasklet::new(task_b);

    let mut executor = Executor::new();
    // executor.enqueue_task(TaskHandle(&taska), unsafe {
    //     core::mem::transmute::<_, c_void>(42u32)
    // });
    // executor.enqueue_task(TaskHandle(&taskb), 8u8 as c_void);

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
