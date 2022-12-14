
mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;
pub use address::StepByOne;
pub use address::{PhysAddr, PhysPageNum, VPNRange, VirtAddr, VirtPageNum};
pub use frame_allocator::{frame_alloc, frame_dealloc, FrameTracker};
pub use memory_set::{kernel_token, remap_test};
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{
    translate_va, translated_byte_buffer, translated_ref, translated_refmut, translated_str,
    PageTableEntry,
};
pub use page_table::{PTEFlags, PageTable, UserBuffer};

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}