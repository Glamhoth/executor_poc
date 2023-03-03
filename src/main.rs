#![no_std]
#![no_main]
// #![allow(warnings, unused)]

extern crate atsamx7x_hal as hal;
extern crate panic_semihosting;

mod rtos;

use core::any::Any;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::debug;
use heapless::binary_heap::{BinaryHeap, Max};

use crate::rtos::executor::Executor;
use crate::rtos::task::Task;
use crate::rtos::taskdata::TaskData;
use crate::rtos::tasklet::Tasklet;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct TaskAData {
    counter: u32,
}

impl TaskData for TaskAData {
    // fn as_any_mut(&mut self) -> &mut dyn Any {
    //     self
    // }
}

fn task_a() {}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct TaskBData {
    small_counter: u8,
}

impl TaskData for TaskBData {
    // fn as_any_mut(&mut self) -> &mut dyn Any {
    //     self
    // }
}

fn task_b() {}

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();

    let mut syst = peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(800_000);
    syst.enable_interrupt();
    syst.enable_counter();

    let task_a_data = TaskAData { counter: 0 };
    let task_a = Tasklet::new(task_a_data, &task_a);

    let task_b_data = TaskBData { small_counter: 0 };
    let task_b = Tasklet::new(task_b_data, &task_b);

    let mut queue: BinaryHeap<*dyn Any, Max, 8> = BinaryHeap::new();
    queue.push(task_a.as_any());
    queue.push(task_b.as_any());

    // let mut executor = Executor::new();
    // executor.enqueue_task(task_a);
    // executor.enqueue_task(task_b);

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[exception]
fn SysTick() {}
