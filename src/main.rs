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
use cortex_m_semihosting::{debug, hio, hprintln};

use crate::rtos::channel::Channel;
use crate::rtos::executor::Executor;
use crate::rtos::notifiable::Notifiable;
use crate::rtos::safecell::SafeCell;
use crate::rtos::tasklet::Tasklet;

struct RequesterData {
    do_work: bool,
    request_channel: &'static Channel<()>,
}

fn requester_fn(local_data: &'static SafeCell<RequesterData>, data: Option<bool>) {
    let do_work = &mut local_data.as_ref_mut().do_work;

    match data {
        Some(val) => *do_work = val,
        None => (),
    };

    if *do_work {
        let request_channel = local_data.as_ref().request_channel;
        request_channel.send_data(());
    }
}

struct ContainerData {
    counter: u32,
    raw_data_channel: &'static Channel<u32>,
}

fn container_fn(local_data: &'static SafeCell<ContainerData>, data: ()) {
    let counter = &mut local_data.as_ref_mut().counter;
    *counter += 1;

    let raw_data_channel = local_data.as_ref().raw_data_channel;
    raw_data_channel.send_data(*counter);
}

struct ComputronixData {
    raw_data_counter: u8,
    raw_data: [u32; 4],
    computed_data_channel: &'static Channel<u32>,
}

fn computronix_fn(local_data: &'static SafeCell<ComputronixData>, data: u32) {
    let raw_data_counter = &mut local_data.as_ref_mut().raw_data_counter;
    *raw_data_counter += 1;

    let raw_data = &mut local_data.as_ref_mut().raw_data;
    raw_data[*raw_data_counter as usize - 1] = data;

    if *raw_data_counter == 4 {
        let computed = raw_data.iter().sum();

        let computed_data_channel = local_data.as_ref().computed_data_channel;
        computed_data_channel.send_data(computed);

        *raw_data_counter = 0;
    }
}

struct ReceiverData {
    sum: u32,
    toggle_channel: &'static Channel<Option<bool>>,
}

fn receiver_fn(local_data: &'static SafeCell<ReceiverData>, data: u32) {
    let sum = &mut local_data.as_ref_mut().sum;
    *sum += data;

    if *sum > 5772 {
        hprintln!("Computation completed");

        let toggle_channel = local_data.as_ref().toggle_channel;
        toggle_channel.send_data(Some(false));
    }
}

static TOGGLE_CHANNEL: Channel<Option<bool>> = Channel::new();
static REQUEST_CHANNEL: Channel<()> = Channel::new();
static RAW_DATA_CHANNEL: Channel<u32> = Channel::new();
static COMPUTED_DATA_CHANNEL: Channel<u32> = Channel::new();

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();

    let mut syst = peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(800_000);
    syst.enable_interrupt();
    syst.enable_counter();

    static requester_data: SafeCell<RequesterData> = SafeCell::new(RequesterData {
        do_work: true,
        request_channel: &REQUEST_CHANNEL,
    });
    static requester: Tasklet<RequesterData, Option<bool>> =
        Tasklet::new(&requester_data, requester_fn, 1);
    TOGGLE_CHANNEL.register_task(&requester);

    static container_data: SafeCell<ContainerData> = SafeCell::new(ContainerData {
        counter: 0,
        raw_data_channel: &RAW_DATA_CHANNEL,
    });
    static container: Tasklet<ContainerData, ()> = Tasklet::new(&container_data, container_fn, 2);
    REQUEST_CHANNEL.register_task(&container);

    static computronix_data: SafeCell<ComputronixData> = SafeCell::new(ComputronixData {
        raw_data_counter: 0,
        raw_data: [0; 4],
        computed_data_channel: &COMPUTED_DATA_CHANNEL,
    });
    static computronix: Tasklet<ComputronixData, u32> =
        Tasklet::new(&computronix_data, computronix_fn, 3);
    RAW_DATA_CHANNEL.register_task(&computronix);

    static receiver_data: SafeCell<ReceiverData> = SafeCell::new(ReceiverData {
        sum: 0,
        toggle_channel: &TOGGLE_CHANNEL,
    });
    static receiver: Tasklet<ReceiverData, u32> = Tasklet::new(&receiver_data, receiver_fn, 4);
    COMPUTED_DATA_CHANNEL.register_task(&receiver);

    let mut executor = Executor::new([&requester, &container, &computronix, &receiver]);
    executor.start();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[exception]
fn SysTick() {
    TOGGLE_CHANNEL.send_data(None);
}
