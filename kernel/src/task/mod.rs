mod context;
mod kernel_stack;
mod manager;
mod pid;
mod processor;
mod switch;
mod task;

use alloc::sync::Arc;
use kernel_hal::{VirtAddr, VirtPageNum};
use lazy_static::*;
use task::{TaskControlBlock, TaskStatus};
use crate::mm::MapPermission;

pub use context::TaskContext;
pub use kernel_stack::KernelStack;
pub use manager::{add_task, TASK_MANAGER, find_task};
pub use pid::{pid_alloc, PidHandle};
pub use processor::{
    current_task, current_trap_cx, current_user_token, run_tasks, schedule, take_current_task,
};

use crate::loader::get_app_data_by_name;

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("initproc").unwrap()
    ));
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- hold current PCB lock
    let mut task_inner = task.acquire_inner_lock();
    let task_cx_ptr2 = task_inner.get_task_cx_ptr2();
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current PCB lock

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr2);
}

pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();
    // **** hold current PCB lock
    let mut inner = task.acquire_inner_lock();
    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // do not move to its parent but under initproc

    // ++++++ hold initproc PCB lock here
    {
        let mut initproc_inner = INITPROC.acquire_inner_lock();
        for child in inner.children.iter() {
            child.acquire_inner_lock().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ release parent PCB lock here

    inner.children.clear();
    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // **** release current PCB lock
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let _unused: usize = 0;
    schedule(&_unused as *const _);
}

pub fn find_free_frames(page_num: usize) -> VirtPageNum {
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    task_inner.find_free_frames(page_num)
}

pub fn alloc_new_frames(start: VirtAddr, end: VirtAddr, permission: MapPermission) {
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    task_inner.alloc_new_frames(start, end, permission)
}

pub fn dealloc_frames(start: VirtAddr, end: VirtAddr) {
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    task_inner.dealloc_frames(start, end);
}

pub fn check_allocated(start: VirtAddr, end: VirtAddr) -> bool {
    let task = current_task().unwrap();
    let task_inner = task.acquire_inner_lock();
    task_inner.check_allocated(start, end)
}

pub fn check_all_allocated(start: VirtAddr, end: VirtAddr) -> bool {
    let task = current_task().unwrap();
    let task_inner = task.acquire_inner_lock();
    task_inner.check_all_allocated(start, end)
}
