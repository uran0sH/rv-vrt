use crate::task::suspend_current_and_run_next;
use kernel_hal::sbi::console_getchar;
use super::File;

pub struct Stdin;

pub struct Stdout;

impl File for Stdin {
    fn read(&self, mut buf: crate::mm::UserBuffer) -> usize {
        assert_eq!(buf.len(), 1);
        let mut c: usize;
        loop {
            c = console_getchar();
            if c == 0 {
                suspend_current_and_run_next();
                continue;
            } else {
                break;
            }
        }
        let ch = c as u8;
        unsafe {
            buf.buffers[0].as_mut_ptr().write_volatile(ch);
        }
        1
    }

    fn write(&self, _buf: crate::mm::UserBuffer) -> usize {
        panic!("Cannot write to stdin!");
    }
}

impl File for Stdout {
    fn read(&self, _buf: crate::mm::UserBuffer) -> usize {
        panic!("Cannot read from stdout!");
    }

    fn write(&self, buf: crate::mm::UserBuffer) -> usize {
        for buffer in buf.buffers.iter() {
            print!("{}", core::str::from_utf8(*buffer).unwrap());
        }
        buf.len()
    }
}
