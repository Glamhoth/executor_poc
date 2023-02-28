use core::cmp::Ordering;
use core::fmt::{Debug, Error, Formatter};

use crate::program::taska::TaskA;
use crate::program::taskb::TaskB;
use crate::rtos::cell::SafeCell;
use crate::rtos::task::{Task, TaskList, TaskState};

pub enum MyTasks {
    TaskA(&'static TaskA),
    TaskB(&'static TaskB),
}

impl TaskList for MyTasks {
    fn get_state(&self) -> TaskState {
        match self {
            MyTasks::TaskA(task_a) => task_a.get_state(),
            MyTasks::TaskB(task_b) => task_b.get_state(),
        }
    }

    fn set_state(&self, state: TaskState) {
        match self {
            MyTasks::TaskA(task_a) => task_a.set_state(state),
            MyTasks::TaskB(task_b) => task_b.set_state(state),
        };
    }

    fn get_last_running_time(&self) -> u64 {
        match self {
            MyTasks::TaskA(task_a) => task_a.get_last_running_time(),
            MyTasks::TaskB(task_b) => task_b.get_last_running_time(),
        }
    }

    fn set_last_running_time(&self, time: u64) {
        match self {
            MyTasks::TaskA(task_a) => task_a.set_last_running_time(time),
            MyTasks::TaskB(task_b) => task_b.set_last_running_time(time),
        };
    }

    fn dispatch(&self) {
        match self {
            MyTasks::TaskA(task_a) => task_a.step(),
            MyTasks::TaskB(task_b) => task_b.step(),
        }
    }
}

impl<'a> Debug for MyTasks {
    fn fmt(&self, _: &mut Formatter) -> Result<(), Error> {
        todo!();
    }
}

impl<'a> Ord for MyTasks {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_state = self.get_state();
        let other_state = self.get_state();

        match (self_state, other_state) {
            (TaskState::Ready, TaskState::Running) => return Ordering::Greater,
            (TaskState::Ready, TaskState::Blocked) => return Ordering::Greater,
            (TaskState::Running, TaskState::Ready) => return Ordering::Less,
            (TaskState::Blocked, TaskState::Ready) => return Ordering::Less,
            (_, _) => (),
        };

        let self_last_running_time = self.get_last_running_time();
        let other_last_running_time = other.get_last_running_time();

        if self_last_running_time > other_last_running_time {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
}

impl<'a> PartialOrd for MyTasks {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        todo!();
    }
}

impl<'a> Eq for MyTasks {}

impl<'a> PartialEq for MyTasks {
    fn eq(&self, _: &Self) -> bool {
        todo!();
    }
}
