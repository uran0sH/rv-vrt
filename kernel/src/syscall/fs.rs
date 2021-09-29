use alloc::vec::Vec;

use crate::ipc::MessagePacket;
use crate::mm::{UserBuffer, translated_byte_buffer, translated_str};
use crate::task::{PidHandle, current_task, current_user_token, find_task};
use crate::service::{REGISTRY, Service};

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        // release Task lock manually to avoid deadlock
        file.write(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        // release Task lock manually to avoid deadlock
        drop(inner);
        file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

pub fn sys_channel_write(service_path: *const u8, buf: *const u8, len: usize) -> isize {
    // get current task token
    let token = current_user_token();
    // find the task correspond to service
    let service_path_str = translated_str(token, service_path);
    let pid = REGISTRY.find_task(&Service::new(service_path_str));
    let task = find_task(pid).unwrap();

    let buf_arr = translated_byte_buffer(token, buf, len);
    let mut data: Vec<u8> = Vec::new();
    for buf in buf_arr {
        for v in buf {
            data.push(*v)
        }
    }

    // transfer bytes to MessagePacket
    let message_packet = MessagePacket {
        data,
        handle: PidHandle(current_task().unwrap().pid.0),
    };
    let inner = task.acquire_inner_lock();
    inner.channel.1.write_msg(message_packet);
    0
}

pub fn sys_channel_read(buf: *mut u8, len: usize) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let task_inner = task.acquire_inner_lock();
    let message_packet: MessagePacket;
    if let Some(m) = task_inner.channel.0.read_msg() {
        message_packet = m;
    } else {
        return -1;
    }
    let user_buffer = UserBuffer::new(translated_byte_buffer(token, buf, len));
    let mut iter = user_buffer.into_iter();
    for message_byte in message_packet.data.iter() {
        if let Some(buffer) = iter.next() {
            unsafe {
                *buffer = *message_byte;
            }
            continue;
        } else {
            break;
        }
    }
    0
}
