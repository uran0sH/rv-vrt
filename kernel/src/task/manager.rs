use super::task::TaskControlBlock;
use alloc::{collections::VecDeque, sync::Arc};
use lazy_static::*;
use spin::Mutex;

pub struct TaskManager {
    pub ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }

    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }

    pub fn find_task(&self, pid: usize) -> Option<Arc<TaskControlBlock>> {
        for task in self.ready_queue.iter() {
            if task.pid.0 == pid {
                return Some(Arc::clone(task));
            }
        }
        None
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.lock().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.lock().fetch()
}

pub fn find_task(pid: usize) -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.lock().find_task(pid)
}
