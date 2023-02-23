#![no_std]
#![no_main]
#![allow(warnings, unused)]

mod rtos;
mod tasks;

extern crate atsamx7x_hal as hal;
extern crate panic_semihosting;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::debug;

use crate::rtos::cell::ThinCell;
use crate::rtos::executor::Executor;
use crate::rtos::task::TaskState;
use crate::tasks::mytasks::MyTasks;
use crate::tasks::taska::TaskA;
use crate::tasks::taskb::TaskB;

static mut SYSTEM_TIME: u64 = 0;

static task_a: ThinCell<MyTasks> = ThinCell::new(MyTasks::TaskA(TaskA::new()));
static task_b: ThinCell<MyTasks> = ThinCell::new(MyTasks::TaskB(TaskB::new()));

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();

    let mut syst = peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000);
    syst.enable_interrupt();
    syst.enable_counter();

    let mut executor = Executor::<MyTasks, 8>::new(unsafe { &SYSTEM_TIME });
    executor.register_task(&task_a);
    executor.register_task(&task_b);

    executor.run_next_task();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[exception]
fn SysTick() {
    unsafe { SYSTEM_TIME += 1 as u64 };
}
