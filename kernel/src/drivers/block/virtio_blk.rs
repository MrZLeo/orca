use crate::mm::address::PhysAddr;
use crate::mm::address::PhysPageNum;
use crate::mm::address::StepByOne;
use crate::mm::address::VirtAddr;
use crate::mm::frame_allocator::FrameTracker;
use crate::mm::frame_allocator::{frame_alloc, frame_dealloc};
use crate::mm::memory_set::KERNEL_SPACE;
use crate::mm::page_table::PageTable;
use crate::mm::page_table::UserBuf;

use alloc::vec::Vec;
use easy_fs::BlockDevice;
use spin::Mutex;
use virtio_drivers::{VirtIOBlk, VirtIOHeader};

const VIRTIO0: usize = 0x10001000;

pub struct VirtIOBlock(Mutex<VirtIOBlk<'static>>);

impl VirtIOBlock {
    pub fn new() -> Self {
        Self(Mutex::new(
            VirtIOBlk::new(unsafe { &mut *(VIRTIO0 as *mut VirtIOHeader) }).unwrap(),
        ))
    }
}

impl BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0
            .lock()
            .read_block(block_id, buf)
            .expect("Error when reading VirtIOBlk");
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0
            .lock()
            .write_block(block_id, buf)
            .expect("Error when writing VirtIOBlk")
    }
}

lazy_static! {
    static ref VRING: Mutex<Vec<FrameTracker>> = Mutex::new(Vec::new());
}

#[no_mangle]
pub extern "C" fn virtio_dma_alloc(pages: usize) -> PhysAddr {
    let mut ppn_base = PhysPageNum(0);
    for i in 0..pages {
        let frame = frame_alloc().unwrap();
        if i == 0 {
            // first frame is the base of whole vring
            ppn_base = frame.ppn;
        }
        // we need to obtain a linear frame
        // so when $i, addr of frame is $base + $i
        assert_eq!(frame.ppn.0, ppn_base.0 + i);
        VRING.lock().push(frame);
    }
    ppn_base.into()
}

#[no_mangle]
pub extern "C" fn virtio_dma_dealloc(pa: PhysAddr, pages: usize) -> i32 {
    let mut ppn_base: PhysPageNum = pa.into();
    for _ in 0..pages {
        frame_dealloc(ppn_base);
        ppn_base.step();
    }
    0
}

#[no_mangle]
pub extern "C" fn virtio_phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    VirtAddr(paddr.0)
}
#[no_mangle]
pub extern "C" fn virtio_virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
    PageTable::from_token(KERNEL_SPACE.borrow_mut().token())
        .translate_va(vaddr)
        .unwrap()
}
