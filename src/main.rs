#![no_std]
#![no_main]

mod rtos;
mod tasks;

use panic_halt as _;

use cortex_m::asm::sev;
use cortex_m_rt::entry;
use cortex_m_rt::exception;
use cortex_m_semihosting::debug;

use crate::rtos::executor::Executor;
use crate::rtos::task::TaskState;
use crate::tasks::mytasks::MyTasks;
use crate::tasks::taska::TaskA;
use crate::tasks::taskb::TaskB;

#[entry]
fn main() -> ! {
    let task_a = MyTasks::TaskA(TaskA::new(TaskState::Ready));
    let task_b = MyTasks::TaskB(TaskB::new(TaskState::Ready));

    let mut executor = Executor::new([task_a, task_b]);
    executor.run_next_task();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[exception]
fn SysTick() {
    sev();
}
