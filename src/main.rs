#![no_std]
#![no_main]
#![allow(warnings, unused)]
#![feature(const_trait_impl)]

mod program;
mod rtos;

extern crate atsamx7x_hal as hal;
extern crate panic_semihosting;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::debug;

use crate::rtos::executor::Executor;
use crate::rtos::messagequeue::MessageQueue;
use crate::rtos::queue::Queue;
use crate::rtos::safecell::SafeCell;
use crate::rtos::tasklet::Tasklet;

use crate::program::taska::{task_a, TaskAData};
use crate::program::taskb::{task_b, TaskBData};

static data_queue: MessageQueue<u32, 64> = MessageQueue::new();

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();

    let mut syst = peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(800_000);
    syst.enable_interrupt();
    syst.enable_counter();

    let mut executor: Executor<8> = Executor::new();

    static taska_res: SafeCell<TaskAData> = SafeCell::new(TaskAData {
        counter: 0,
        data_queue: &data_queue,
    });
    static taska: Tasklet<TaskAData> = Tasklet::new(&taska_res, &task_a);

    static taskb_res: SafeCell<TaskBData> = SafeCell::new(TaskBData {
        small_counter: 0,
        data_queue: &data_queue,
    });
    static taskb: Tasklet<TaskBData> = Tasklet::new(&taskb_res, &task_b);

    executor.enqueue_task(&taska);
    executor.enqueue_task(&taskb);

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
