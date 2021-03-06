use alloc::vec::Vec;
use alloc::{string::String, vec};
use bitflags::bitflags;

use super::address::PhysAddr;
use super::{
    address::{PhysPageNum, StepByOne, VirtAddr, VirtPageNum},
    frame_allocator::{frame_alloc, FrameTracker},
};

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        Self {
            bits: (ppn.0 << 10) | flags.bits as usize,
        }
    }

    pub fn empty() -> Self {
        Self { bits: 0 }
    }

    pub fn ppn(&self) -> PhysPageNum {
        ((self.bits >> 10) & ((1usize << 44) - 1)).into()
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }

    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }

    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }

    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
}

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let frame = frame_alloc().unwrap();
        Self {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }

    pub fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        for (i, &idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.pte_array()[idx];
            if i == 2 {
                return Some(pte);
            }
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        None
    }

    pub fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        for (i, &idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.pte_array()[idx];
            if i == 2 {
                return Some(pte);
            }
            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
        }
        None
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmap", vpn);
        *pte = PageTableEntry::empty();
    }

    pub fn from_token(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1 << 44) - 1)),
            frames: Vec::new(),
        }
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| *pte)
    }

    pub fn token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }

    pub fn translate_va(&self, va: VirtAddr) -> Option<PhysAddr> {
        self.find_pte(va.clone().floor()).map(|pte| {
            let aligned_pa: PhysAddr = pte.ppn().into();
            let offset = va.offset();
            let addr = usize::from(aligned_pa) + offset;
            addr.into()
        })
    }
}

pub fn translated_byte_buffer(token: usize, ptr: *const u8, len: usize) -> Vec<&'static mut [u8]> {
    let page_table = PageTable::from_token(token);
    let mut start = ptr as usize;
    let end = start + len;
    let mut v = Vec::new();
    while start < end {
        let va_start = VirtAddr::from(start);
        let mut vpn = va_start.floor();
        let ppn = page_table.translate(vpn).unwrap().ppn();
        vpn.step();
        let mut va_end: VirtAddr = vpn.into();
        va_end = va_end.min(VirtAddr::from(end));
        if va_end.offset() == 0 {
            v.push(&mut ppn.bytes_array()[va_start.offset()..]);
        } else {
            v.push(&mut ppn.bytes_array()[va_start.offset()..va_end.offset()]);
        }
        start = va_end.into();
    }
    v
}

pub fn translated_str(token: usize, ptr: *const u8) -> String {
    let page_table = PageTable::from_token(token);
    let mut start = ptr as usize;
    let mut res = String::new();
    loop {
        let ch: u8 = *(page_table
            .translate_va(VirtAddr::from(start))
            .unwrap()
            .as_mut());

        // get '\0', the end of string
        if ch == 0 {
            break;
        } else {
            res.push(ch as char);
            start += 1;
        }
    }

    res
}

pub fn translated_refmut<T>(token: usize, ptr: *const T) -> &'static mut T {
    let page_table = PageTable::from_token(token);
    let va = ptr as usize;

    page_table.translate_va(va.into()).unwrap().as_mut()
}

pub struct UserBuf {
    pub buffers: Vec<&'static mut [u8]>,
}

impl UserBuf {
    pub fn new(buffers: Vec<&'static mut [u8]>) -> Self {
        Self { buffers }
    }

    pub fn len(&self) -> usize {
        let mut total = 0;
        for buf in self.buffers.iter() {
            total += buf.len();
        }
        total
    }
}
