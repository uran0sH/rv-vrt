const SYSCALL_CLOSE: usize = 57;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_WAITPID: usize = 260;
const SYSCALL_CREATE_TASK: usize = 400;
const SYSCALL_MMAP_CREATE: usize = 401;
const SYSCALL_SERVICE_REGISTER: usize = 500;
const SYSCALL_CHANNEL_READ: usize = 501;
const SYSCALL_CHANNEL_WRITE: usize = 502;

mod fs;
mod process;

use fs::*;
use process::*;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_CLOSE => sys_close(args[0]),
        SYSCALL_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_MMAP => sys_mmap(args[0], args[1], args[2]),
        SYSCALL_MUNMAP => sys_munmap(args[0], args[1]),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_FORK => sys_fork(),
        SYSCALL_EXEC => sys_exec(args[0] as *const u8),
        SYSCALL_WAITPID => sys_waitpid(args[0] as isize, args[1] as *mut i32),
        SYSCALL_CREATE_TASK => sys_create_task(args[0] as *const u8),
        SYSCALL_MMAP_CREATE => sys_mmap_create(args[0], args[1]),
        SYSCALL_CHANNEL_READ => sys_channel_read(args[0] as *mut u8, args[1]),
        SYSCALL_CHANNEL_WRITE => sys_channel_write(args[0] as *const u8, args[1] as *const u8, args[2]),
        SYSCALL_SERVICE_REGISTER => sys_register(args[0] as *const u8, args[1] as *const u8),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
