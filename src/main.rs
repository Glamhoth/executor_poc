#![no_std]
#![no_main]

mod rtos;
mod tasks;

// use panic_halt as _;

// extern crate atsamx7x_hal as hal;
extern crate panic_semihosting;

use cortex_m::asm::sev;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_rt::exception;
use cortex_m_semihosting::debug;
// use hal::ehal::watchdog::WatchdogDisable;
// use hal::target_device;

use crate::rtos::executor::Executor;
use crate::rtos::task::TaskState;
use crate::tasks::mytasks::MyTasks;
use crate::tasks::taska::TaskA;
use crate::tasks::taskb::TaskB;

static mut SYSTEM_TIME: u64 = 0;

#[entry]
fn main() -> ! {
    {
        // let peripherals = target_device::Peripherals::take().unwrap();
        // let wdt = peripherals.WDT;
        // hal::watchdog::Watchdog::new(wdt).disable();
    }

    {
        let peripherals = cortex_m::Peripherals::take().unwrap();

        let mut syst = peripherals.SYST;
        syst.set_clock_source(SystClkSource::Core);
        syst.set_reload(8_000_000);
        syst.enable_interrupt();
        syst.enable_counter();
    }

    let task_a = MyTasks::TaskA(TaskA::new(TaskState::Ready));
    let task_b = MyTasks::TaskB(TaskB::new(TaskState::Ready));

    let mut executor = Executor::new([task_a, task_b], unsafe { &SYSTEM_TIME });
    executor.run_next_task();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[exception]
fn SysTick() {
    // unsafe { SYSTEM_TIME.wrapping_add(SYST::get_current() as u64) };
    unsafe { SYSTEM_TIME += 1 as u64 };

    sev();
}
