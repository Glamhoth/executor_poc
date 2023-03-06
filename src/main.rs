#![no_std]
#![no_main]
#![feature(const_trait_impl)]
// #![allow(warnings, unused)]

extern crate atsamx7x_hal as hal;
extern crate panic_semihosting;

mod rtos;

use core::default::Default;
use core::fmt::Write;

use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{debug, hio, hprintln};
use heapless::binary_heap::{BinaryHeap, Max};

use crate::rtos::executor::Executor;
use crate::rtos::safecell::SafeCell;
use crate::rtos::task::Task;
use crate::rtos::taskdata::TaskData;
use crate::rtos::tasklet::Tasklet;

struct TaskAData {
    counter: u32,
}

impl const Default for TaskAData {
    fn default() -> Self {
        TaskAData { counter: 11 }
    }
}

impl TaskData for TaskAData {}

struct TaskBData {
    small_counter: u8,
}

impl const Default for TaskBData {
    fn default() -> Self {
        TaskBData { small_counter: 22 }
    }
}

fn task_a(data: &SafeCell<TaskAData>) {
    let mut counter = data.as_ref_mut().counter;
    counter += 1;

    let mut stdout = hio::hstdout().unwrap();
    write!(stdout, "TaskA: {}\n", counter).unwrap();
}

fn task_b(data: &SafeCell<TaskBData>) {
    let mut counter = data.as_ref_mut().small_counter;
    counter += 1;

    let mut stdout = hio::hstdout().unwrap();
    write!(stdout, "TaskB: {}\n", counter).unwrap();
}

impl TaskData for TaskBData {}

#[entry]
fn main() -> ! {
    static taska: Tasklet<TaskAData> = Tasklet::new(&task_a);
    static taskb: Tasklet<TaskBData> = Tasklet::new(&task_b);

    let mut executor = Executor::<8>::new();

    executor.enqueue_task(&taska);
    executor.enqueue_task(&taskb);

    // let mut queue: BinaryHeap<*const dyn Task, Max, 8> = BinaryHeap::new();
    // queue.push(taska.as_task());
    // queue.push(taskb.as_task());

    // for task in &queue {
    //     hprintln!("{}", unsafe { (**task).get_last_running_time() });
    // }

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[exception]
fn SysTick() {}
