use super::address::PhysAddr;
use super::address::PhysPageNum;
use crate::config::ekernel;
use crate::config::MEMORY_END;
use crate::sync::UniProcSafeCell;
use alloc::vec::Vec;
use lazy_static::lazy_static;

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

/// # physical frame allocator
/// alloc physical frame from ram
///
/// ## alloc
/// @return `Option<PhysPageNum>`
///
/// We use a simple stack to alloc frame.
/// - if user return frame, push it into a vector
/// - if there is no frame that return, from user, `cur++`
/// - if there is some frames released, use it from vector
///
/// ## dealloc
/// release PhysPageNum
///
pub struct StackFrameAllocator {
    cur: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        self.cur = l.0;
        self.end = r.0;
    }
    pub fn empty(&self) -> bool {
        self.cur == self.end
    }

    fn is_valid(&self, ppn: PhysPageNum) -> bool {
        let ppn = ppn.0;
        ppn < self.cur && !self.recycled.iter().any(|v| *v == ppn)
    }
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            cur: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else if self.empty() {
            None
        } else {
            self.cur += 1;
            Some((self.cur - 1).into())
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        if self.is_valid(ppn) {
            let ppn = ppn.0;
            self.recycled.push(ppn);
        } else {
            panic!("Frame ppn={:#x} hasn't been allocated", ppn.0);
        }
    }
}

type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    pub static ref FRAME_ALLOCATOR: UniProcSafeCell<FrameAllocatorImpl> =
        UniProcSafeCell::new(FrameAllocatorImpl::new());
}

// pub interface
pub fn init() {
    FRAME_ALLOCATOR.borrow_mut().init(
        PhysAddr::from(ekernel as usize).ceil(),
        PhysAddr::from(MEMORY_END).floor(),
    )
}

pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR.borrow_mut().alloc().map(FrameTracker::new)
}

// do not use outside
pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.borrow_mut().dealloc(ppn);
}

/// wraper of PhysPageNum
#[derive(Debug)]
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    fn new(ppn: PhysPageNum) -> Self {
        let bytes_arr = ppn.bytes_array();
        for i in bytes_arr {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}
