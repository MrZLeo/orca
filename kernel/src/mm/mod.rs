pub mod address;
pub mod frame_allocator;
mod heap_allocator;
pub mod memory_set;
pub mod page_table;

use self::memory_set::KERNEL_SPACE;

pub fn init() {
    heap_allocator::init();
    frame_allocator::init();
    KERNEL_SPACE.borrow_mut().activate();
    debug!("mm:init end");
}
