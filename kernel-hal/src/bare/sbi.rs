#![allow(unused)]

pub(crate) const SBI_SET_TIMER: usize = 0;
pub(crate) const SBI_CONSOLE_PUTCHAR: usize = 1;
pub(crate) const SBI_CONSOLE_GETCHAR: usize = 2;
pub(crate) const SBI_CLEAR_IPI: usize = 3;
pub(crate) const SBI_SEND_IPI: usize = 4;
pub(crate) const SBI_REMOTE_FENCE_I: usize = 5;
pub(crate) const SBI_REMOTE_SFENCE_VMA: usize = 6;
pub(crate) const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
pub(crate) const SBI_SHUTDOWN: usize = 8;

#[inline(always)]
pub(crate) fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"
            : "volatile"
        );
    }
    ret
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}

pub fn set_timer(timer: usize) {
    sbi_call(SBI_SET_TIMER, timer, 0, 0);
}

hal_fn_impl! {
    impl mod crate::hal_fn::sbi {
        fn console_putchar(c: usize) {
            sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
        }

        fn console_getchar() -> usize {
            sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
        }

        fn shutdown() -> ! {
            sbi_call(SBI_SHUTDOWN, 0, 0, 0);
            panic!("It should shutdown!");
        }
    }
}
