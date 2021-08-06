#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod config;
mod fs;
mod lang_items;
mod loader;
mod riscv_mm;
mod sbi;
mod syscall;
mod task;
mod timer;
mod trap;

extern crate alloc;

#[macro_use]
extern crate bitflags;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

#[no_mangle]
pub fn rust_main() -> ! {
    println!("Hello RV-VRT");
    clear_bss();
    riscv_mm::init();
    riscv_mm::remap_test();
    task::add_initproc();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    loader::list_apps();
    task::run_tasks();
    panic!("Shutdown machine!");
}
