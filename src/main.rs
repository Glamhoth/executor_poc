#![no_std]
#![no_main]
#![feature(const_trait_impl)]
// #![allow(warnings, unused)]

extern crate atsamx7x_hal as hal;
extern crate panic_semihosting;

mod rtos;

use core::default::Default;

use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{debug, hprintln};
use heapless::binary_heap::{BinaryHeap, Max};

use crate::rtos::task::Task;
use crate::rtos::taskdata::TaskData;
use crate::rtos::tasklet::Tasklet;

struct TaskAData {
    counter: u32,
}

impl const Default for TaskAData {
    fn default() -> Self {
        TaskAData {
            counter: 11
        }
    }
}

impl TaskData for TaskAData {}

struct TaskBData {
    small_counter: u8,
}

impl const Default for TaskBData {
    fn default() -> Self {
        TaskBData {
            small_counter: 22
        }
    }
}

impl TaskData for TaskBData {}

#[entry]
fn main() -> ! {
    static task_a: Tasklet<TaskAData> = Tasklet::new(42);
    static task_b: Tasklet<TaskBData> = Tasklet::new(21);

    let mut queue: BinaryHeap<*const dyn Task, Max, 8> = BinaryHeap::new();
    queue.push(task_a.as_task());
    queue.push(task_b.as_task());

    for task in &queue {
        hprintln!("{}", unsafe { (**task).get_last_running_time() });
    }

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[exception]
fn SysTick() {}
