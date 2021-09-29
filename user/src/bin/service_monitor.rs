#![no_std]
#![no_main]

use user_lib::channel_read;

#[macro_use]
extern crate user_lib;

#[no_mangle]
pub fn main() -> i32 {
    loop {
        let mut buf = [0u8; 27];
        let result_code = channel_read(&mut buf);
        if result_code == -1 {
            continue;
        }
        println!("service_monitor receive = {:?}", buf);
    }
    0
}