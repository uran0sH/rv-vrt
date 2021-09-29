use crate::common::mm::PhysPageNum;

hal_fn_def! {
    pub mod sbi {
        pub fn console_putchar(c: usize);
        pub fn console_getchar() -> usize;
        pub fn shutdown() -> !;
    }

    pub mod timer {
        pub fn set_next_trigger();
        pub fn get_time_ms() -> usize;
        pub fn get_time() -> usize;
    }

    pub mod vm {
        pub fn calculate_root_ppn(satp: usize) -> PhysPageNum;
        pub fn get_vmtoken(ppn: usize) -> usize;
        pub fn activate_paging(satp: usize);
    }
}
