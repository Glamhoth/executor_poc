#![no_std]
#![no_main]
#![allow(warnings, unused)]
#![feature(trait_upcasting)]

mod rtos;

extern crate atsamx7x_hal as hal;
extern crate panic_semihosting;

use core::fmt::Write;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{debug, hio};

use crate::rtos::channel::Channel;
use crate::rtos::executor::Executor;
use crate::rtos::notifiable::Notifiable;
use crate::rtos::safecell::SafeCell;
use crate::rtos::tasklet::Tasklet;

struct FizzerData {
    counter: u32,
}

struct BuzzerData {
    counter: u32,
}

struct StubberData {
    counter: u32,
}

struct NewlinerData {
    counter: u32,
}

fn fizzer_fn(local_data: &'static SafeCell<FizzerData>, data: u32) {
    let counter = &mut local_data.as_ref_mut().counter;

    if data % 3 == 0 {
        let mut stdout = hio::hstdout().unwrap();
        write!(stdout, "Fizz");
    }

    *counter += 1;
}

fn buzzer_fn(local_data: &'static SafeCell<BuzzerData>, data: u32) {
    let counter = &mut local_data.as_ref_mut().counter;

    if data % 5 == 0 {
        let mut stdout = hio::hstdout().unwrap();
        write!(stdout, "Buzz");
    }

    *counter += 1;
}

fn stubber_fn(local_data: &'static SafeCell<StubberData>, data: u32) {
    let counter = &mut local_data.as_ref_mut().counter;

    if data % 3 != 0 && data % 5 != 0 {
        let mut stdout = hio::hstdout().unwrap();
        write!(stdout, "{}", data);
    }

    *counter += 1;
}

fn newliner_fn(local_data: &'static SafeCell<NewlinerData>, data: u32) {
    let counter = &mut local_data.as_ref_mut().counter;

    let mut stdout = hio::hstdout().unwrap();
    write!(stdout, "\n");

    *counter += 1;
}

static executor: Executor = Executor::new();
static channel: Channel<u32> = Channel::new(&executor);

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();

    let mut syst = peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(800_000);
    syst.enable_interrupt();
    syst.enable_counter();

    static fizzer_data: SafeCell<FizzerData> = SafeCell::new(FizzerData { counter: 0 });
    static fizzer: Tasklet<FizzerData, u32> = Tasklet::new(&fizzer_data, fizzer_fn, 4);
    channel.register_task(&fizzer);

    static buzzer_data: SafeCell<BuzzerData> = SafeCell::new(BuzzerData { counter: 0 });
    static buzzer: Tasklet<BuzzerData, u32> = Tasklet::new(&buzzer_data, buzzer_fn, 3);
    channel.register_task(&buzzer);

    static stubber_data: SafeCell<StubberData> = SafeCell::new(StubberData { counter: 0 });
    static stubber: Tasklet<StubberData, u32> = Tasklet::new(&stubber_data, stubber_fn, 2);
    channel.register_task(&stubber);

    static newliner_data: SafeCell<NewlinerData> = SafeCell::new(NewlinerData { counter: 0 });
    static newliner: Tasklet<NewlinerData, u32> = Tasklet::new(&newliner_data, newliner_fn, 1);
    channel.register_task(&newliner);

    channel.send_data(1);

    executor.start();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[exception]
fn SysTick() {
    static mut val: u32 = 1;

    *val += 1;
    channel.send_data(*val);
}
