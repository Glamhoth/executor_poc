#![no_std]
#![no_main]
#![allow(warnings, unused)]

mod program;
mod rtos;

extern crate atsamx7x_hal as hal;
extern crate panic_semihosting;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::debug;

use crate::program::mytasks::MyTasks;
use crate::program::taska::TaskA;
use crate::program::taskb::TaskB;
use crate::rtos::cell::SafeCell;
use crate::rtos::executor::Executor;
use crate::rtos::messagequeue::MessageQueue;
use crate::rtos::queue::Queue;
use crate::rtos::task::TaskState;

static data_queue: MessageQueue<MyTasks, u32, 64> = MessageQueue::new();

static task_a: TaskA = TaskA::new(&data_queue);
static task_b: TaskB = TaskB::new(&data_queue);

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();

    let mut syst = peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(800_000);
    syst.enable_interrupt();
    syst.enable_counter();

    data_queue.enqueue(0);
    data_queue.enqueue(1);
    data_queue.enqueue(2);

    let mut executor: Executor<MyTasks, 8> = Executor::new();

    executor.enqueue_task(MyTasks::TaskA(&task_a));
    executor.enqueue_task(MyTasks::TaskB(&task_b));

    executor.start();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[exception]
fn SysTick() {
    static mut val: u32 = 0;

    match data_queue.enqueue(*val) {
        Ok(_) => {
            *val += 1;
            data_queue.notify();
        }
        Err(_) => (),
    }
}
