#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{mmap_create, munmap};

/*
理想结果：输出 Test 04_1 OK!
*/

#[no_mangle]
fn main() -> i32 {
    let len: usize = 4096;
    let prot: usize = 3;
    // assert_eq!(len as isize, mmap_create(len, prot));
    let start = mmap_create(len, prot) as usize;
    for i in start..(start + len) {
        let addr: *mut u8 = i as *mut u8;
        unsafe {
            *addr = i as u8;
        }
    }
    for i in start..(start + len) {
        let addr: *mut u8 = i as *mut u8;
        unsafe {
            assert_eq!(*addr, i as u8);
        }
    }
    assert_eq!(munmap(start, len), len as isize);
    println!("Test mmap_create0 OK!");
    0
}