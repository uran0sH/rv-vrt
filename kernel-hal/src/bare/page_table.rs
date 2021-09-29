use riscv::register::satp;

use crate::PhysPageNum;

hal_fn_impl! {
    impl mod crate::hal_fn::vm {
        fn calculate_root_ppn(satp: usize) -> PhysPageNum {
            PhysPageNum::from(satp & ((1usize << 44) - 1))
        }

        fn get_vmtoken(ppn: usize) -> usize {
            8usize << 60 | ppn
        }

        fn activate_paging(satp: usize)  {
            unsafe {
                satp::write(satp);
                llvm_asm!("sfence.vma" :::: "volatile");
            }
        }
    }
}

