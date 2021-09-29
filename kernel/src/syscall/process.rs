use alloc::sync::Arc;
use kernel_hal::VirtAddr;

use crate::mm::{translated_refmut, translated_str};
use crate::service::Service;
use crate::task::{PidHandle, alloc_new_frames, check_all_allocated, check_allocated, dealloc_frames, find_free_frames};
use kernel_hal::{timer::get_time_ms};
use crate::{
    loader::get_app_data_by_name,
    mm::MapPermission,
    task::{
        add_task, current_task, current_user_token, exit_current_and_run_next,
        suspend_current_and_run_next,
    },
    service::REGISTRY,
};

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

const MAX_ALLOC_MEMORY: usize = 1024 * 1024 * 1024;

pub fn sys_mmap_create(len: usize, prot: usize) -> isize {
    // check port
    if prot & !0x7 != 0 || prot & 0x7 == 0 {
        return -1;
    }
    // check len
    if len > MAX_ALLOC_MEMORY {
        return -1;
    }
    let start = find_free_frames(len);
    let start_va: VirtAddr = start.into();
    let per = MapPermission::from_bits(((prot << 1) | 16) as u8).unwrap();
    alloc_new_frames(start_va, (start_va.0 + len).into(), per);
    start_va.0 as isize
}

pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    // check port
    if prot & !0x7 != 0 || prot & 0x7 == 0 {
        return -1;
    }
    // check len
    if len > MAX_ALLOC_MEMORY {
        return -1;
    }
    let virt_addr_start: VirtAddr = start.into();
    let virt_addr_end: VirtAddr = (start + len).into();
    // check addr align
    if !virt_addr_start.aligned() {
        return -1;
    }
    // check if allocated
    if check_allocated(virt_addr_start, virt_addr_end) {
        return -1;
    }
    // allocate
    let per = MapPermission::from_bits(((prot << 1) | 16) as u8).unwrap();
    alloc_new_frames(virt_addr_start, virt_addr_end, per);
    ((virt_addr_end.ceil().0 - virt_addr_start.floor().0) * 4096) as isize
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    let virt_addr_start: VirtAddr = start.into();
    let virt_addr_end: VirtAddr = (start + len).into();
    if !virt_addr_start.aligned() {
        return -1;
    }
    // check
    if !check_all_allocated(virt_addr_start, virt_addr_end) {
        return -1;
    }
    dealloc_frames(virt_addr_start, virt_addr_end);
    ((virt_addr_end.ceil().0 - virt_addr_start.floor().0) * 4096) as isize
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.acquire_inner_lock().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = current_task().unwrap();
    // find a child process

    // ---- hold current PCB lock
    let mut inner = task.acquire_inner_lock();
    if inner
        .children
        .iter()
        .find(|p| pid == -1 || pid as usize == p.getpid())
        .is_none()
    {
        return -1;
        // ---- release current PCB lock
    }
    let pair = inner.children.iter().enumerate().find(|(_, p)| {
        // ++++ temporarily hold child PCB lock
        p.acquire_inner_lock().is_zombie() && (pid == -1 || pid as usize == p.getpid())
        // ++++ release child PCB lock
    });
    if let Some((idx, _)) = pair {
        let child = inner.children.remove(idx);
        // confirm that child will be deallocated after removing from children list
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.getpid();
        // ++++ temporarily hold child lock
        let exit_code = child.acquire_inner_lock().exit_code;
        // ++++ release child PCB lock
        *translated_refmut(inner.memory_set.token(), exit_code_ptr) = exit_code;
        found_pid as isize
    } else {
        -2
    }
    // ---- release current PCB lock automatically
}

pub fn sys_getpid() -> isize {
    current_task().unwrap().pid.0 as isize
}

pub fn sys_create_task(file: *const u8) -> isize {
    let token = current_user_token();
    let path = translated_str(token, file);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        let next = task.create(data);
        let pid = next.pid.0 as isize;
        add_task(next);
        pid
    } else {
        -1
    }
}

pub fn sys_register(file: *const u8, serivce: *const u8) -> isize {
    let pid = sys_create_task(file);
    if pid == -1 {
        return -1
    }
    let token = current_user_token();
    let service_path = translated_str(token, serivce);
    REGISTRY.register(&PidHandle(pid as usize), &Service::new(service_path));
    pid
} 
