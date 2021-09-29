mod heap_allocator;
mod memory_set;
mod frame_allocator;
mod page_table;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}

pub use memory_set::remap_test;
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{
    check_address_valid, translated_byte_buffer, translated_refmut, translated_str, UserBuffer,
    UserBufferIterator,
};