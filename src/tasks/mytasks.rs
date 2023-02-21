use core::cmp::Ordering;

use crate::rtos::task::{Task, TaskState};
use crate::rtos::tasklist::TaskList;
use crate::tasks::taska::TaskA;
use crate::tasks::taskb::TaskB;

#[derive(Debug, Eq, Ord)]
pub enum MyTasks {
    TaskA(TaskA),
    TaskB(TaskB),
}

impl TaskList for MyTasks {
    fn get_state(&self) -> &TaskState {
        match self {
            MyTasks::TaskA(task_a) => &task_a.get_state(),
            MyTasks::TaskB(task_b) => &task_b.get_state(),
        }
    }

    fn set_state(&mut self, state: TaskState) {
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

    fn set_last_running_time(&mut self, time: u64) {
        match self {
            MyTasks::TaskA(task_a) => task_a.set_last_running_time(time),
            MyTasks::TaskB(task_b) => task_b.set_last_running_time(time),
        };
    }

    fn dispatch(&mut self) -> TaskState {
        match self {
            MyTasks::TaskA(task_a) => task_a.step(),
            MyTasks::TaskB(task_b) => task_b.step(),
        }
    }
}

impl<'a> PartialOrd for MyTasks {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_state = self.get_state();
        let other_state = other.get_state();

        match (self_state, other_state) {
            // (TaskState::Ready, TaskState::Ready) => Some(Ordering::Equal),
            (TaskState::Ready, TaskState::Running) => return Some(Ordering::Greater),
            (TaskState::Ready, TaskState::Blocked) => return Some(Ordering::Greater),
            (TaskState::Running, TaskState::Ready) => return Some(Ordering::Less),
            // (TaskState::Running, TaskState::Running) => Some(Ordering::Equal),
            // (TaskState::Running, TaskState::Blocked) => Some(Ordering::Equal),
            (TaskState::Blocked, TaskState::Ready) => return Some(Ordering::Less),
            // (TaskState::Blocked, TaskState::Running) => Some(Ordering::Equal),
            // (TaskState::Blocked, TaskState::Blocked) => Some(Ordering::Equal),
            (_, _) => None::<Ordering>,
        };

        let self_last_running_time = self.get_last_running_time();
        let other_last_running_time = other.get_last_running_time();

        if self_last_running_time > other_last_running_time {
            return Some(Ordering::Less);
        } else {
            return Some(Ordering::Greater);
        }
    }
}

impl<'a> PartialEq for MyTasks {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
