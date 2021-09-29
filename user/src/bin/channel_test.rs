#![no_std]
#![no_main]

use user_lib::{channel_write, register, waitpid};

#[macro_use]
extern crate user_lib;

#[no_mangle]
pub fn main() -> i32 {
    let cpid = register("service_monitor\0", "com.test.sensor");
    if cpid == -1 {
        println!("create failed");
        return 0;
    }
    let buf = ['a' as u8; 27];
    channel_write("com.test.sensor", &buf);
    loop {
        if waitpid(cpid as usize, &mut 0) == -1 {
            break;
        }
    }
    0
}