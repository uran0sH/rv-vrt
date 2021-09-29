const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_WAITPID: usize = 260;
const SYSCALL_CREATE_TASK: usize = 400;
const SYSCALL_MMAP_CREATE: usize = 401;
const SYSCALL_SERVICE_REGISTER: usize = 500;
const SYSCALL_CHANNEL_READ: usize = 501;
const SYSCALL_CHANNEL_WRITE: usize = 502;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret
}

pub fn sys_close(fd: usize) -> isize {
    syscall(SYSCALL_CLOSE, [fd, 0, 0])
}

pub fn sys_pipe(pipe: &mut [usize]) -> isize {
    syscall(SYSCALL_PIPE, [pipe.as_mut_ptr() as usize, 0, 0])
}

pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(SYSCALL_READ, [fd, buffer.as_mut_ptr() as usize, buffer.len()])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> ! {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0]);
    panic!("sys_exit never returns!");
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}

pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, [0, 0, 0])
}

pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, [0, 0, 0])
}

pub fn sys_exec(path: &str) -> isize {
    syscall(SYSCALL_EXEC, [path.as_ptr() as usize, 0, 0])
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    syscall(SYSCALL_WAITPID, [pid as usize, exit_code as usize, 0])
}

pub fn sys_create_task(path: &str) -> isize {
    syscall(SYSCALL_CREATE_TASK, [path.as_ptr() as usize, 0, 0])
}

pub fn sys_mmap_create(len: usize, port: usize) -> isize {
    syscall(SYSCALL_MMAP_CREATE, [len, port, 0])
}

pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    syscall(SYSCALL_MMAP, [start, len, prot])
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    syscall(SYSCALL_MUNMAP, [start, len, 0])
}

pub fn sys_channel_read(buf: &mut [u8], len: usize) -> isize {
    syscall(SYSCALL_CHANNEL_READ, [buf.as_mut_ptr() as usize, len, 0])
}

pub fn sys_channel_write(path: &str, buf: &[u8], len: usize) -> isize {
    syscall(SYSCALL_CHANNEL_WRITE, [path.as_ptr() as usize, buf.as_ptr() as usize, len])
}

pub fn sys_register(file: &str, service: &str) -> isize {
    syscall(SYSCALL_SERVICE_REGISTER, [file.as_ptr() as usize, service.as_ptr() as usize, 0])
}