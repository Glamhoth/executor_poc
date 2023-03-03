#![no_std]
#![no_main]
// #![allow(warnings, unused)]

extern crate atsamx7x_hal as hal;
extern crate panic_semihosting;

mod rtos;

use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{debug, hprintln};
use heapless::binary_heap::{BinaryHeap, Max};

use crate::rtos::task::Task;

struct TaskA {
    last_running_time: u32,
}

impl TaskA {
    pub const fn new() -> Self {
        TaskA {
            last_running_time: 42,
        }
    }
}

impl Task for TaskA {
    fn get_last_running_time(&self) -> u32 {
        self.last_running_time
    }
}

struct TaskB {
    last_running_time: u32,
}

impl TaskB {
    pub const fn new() -> Self {
        TaskB {
            last_running_time: 21,
        }
    }
}

impl Task for TaskB {
    fn get_last_running_time(&self) -> u32 {
        self.last_running_time
    }
}

#[entry]
fn main() -> ! {
    static task_a: TaskA = TaskA::new();
    static task_b: TaskB = TaskB::new();

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
