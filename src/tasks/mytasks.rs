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
    fn state(&self) -> &TaskState {
        match self {
            MyTasks::TaskA(task_a) => &task_a.state(),
            MyTasks::TaskB(task_b) => &task_b.state(),
        }
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
        let self_state = self.state();
        let other_state = other.state();

        match (self_state, other_state) {
            (TaskState::Ready, TaskState::Ready) => Some(Ordering::Equal),
            (TaskState::Ready, TaskState::Running) => Some(Ordering::Greater),
            (TaskState::Ready, TaskState::Blocked) => Some(Ordering::Greater),
            (TaskState::Running, TaskState::Ready) => Some(Ordering::Less),
            (TaskState::Running, TaskState::Running) => Some(Ordering::Equal),
            (TaskState::Running, TaskState::Blocked) => Some(Ordering::Equal),
            (TaskState::Blocked, TaskState::Ready) => Some(Ordering::Less),
            (TaskState::Blocked, TaskState::Running) => Some(Ordering::Equal),
            (TaskState::Blocked, TaskState::Blocked) => Some(Ordering::Equal),
        }
    }
}

impl<'a> PartialEq for MyTasks {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
