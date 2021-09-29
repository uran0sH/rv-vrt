#![no_std]
#![no_main]
#![feature(
    llvm_asm,
    global_asm,
    lang_items,
    panic_info_message,
    alloc_error_handler,
    get_mut_unchecked,
)]
use core::panic::PanicInfo;

extern crate alloc;
extern crate kernel_hal;
extern crate hashbrown;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod console;
mod mm;
mod task;
mod trap;
mod loader;
mod fs;
mod ipc;
mod registry;
mod syscall;

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
pub fn rust_main(_hartid: usize, _dtb_pa: usize) -> ! {
    println!("Hello RV-VRT");
    clear_bss();
    mm::init();
    mm::remap_test();
    task::add_initproc();
    trap::init();
    trap::enable_timer_interrupt();
    kernel_hal::timer::set_next_trigger();
    loader::list_apps();
    task::run_tasks();
    kernel_hal::sbi::shutdown()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    kernel_hal::sbi::shutdown()
}
