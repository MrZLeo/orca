//! Definition of address
//! - `PhysAddr`: actual physical address
//! - `VirtAddr`: virtual address
//! - `PhysPageNum`
//! - `VirtPageNum`

use super::page_table::PageTableEntry;
use crate::config::{PAGE_OFFSET, PAGE_SIZE};
use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct PhysAddr(pub usize);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct VirtAddr(pub usize);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct PhysPageNum(pub usize);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct VirtPageNum(pub usize);

const PA_WIDTH_SV39: usize = 56;
const VA_WIDTH_SV39: usize = 39;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_OFFSET;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_OFFSET;

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self({ v & ((1 << PA_WIDTH_SV39) - 1) })
    }
}

impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {
        Self({ v & ((1 << PPN_WIDTH_SV39) - 1) })
    }
}

impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        Self({ v & ((1 << VA_WIDTH_SV39) - 1) })
    }
}

impl From<usize> for VirtPageNum {
    fn from(v: usize) -> Self {
        Self({ v & ((1 << VPN_WIDTH_SV39) - 1) })
    }
}

impl From<PhysAddr> for usize {
    fn from(pa: PhysAddr) -> Self {
        pa.0
    }
}

impl From<PhysPageNum> for usize {
    fn from(ppn: PhysPageNum) -> Self {
        ppn.0
    }
}

impl From<VirtAddr> for usize {
    fn from(va: VirtAddr) -> Self {
        va.0
    }
}

impl From<VirtPageNum> for usize {
    fn from(vpn: VirtPageNum) -> Self {
        vpn.0
    }
}

impl PhysAddr {
    /// - PAGE_SIZE = 4K
    /// => PAGE_SIZE - 1 get 0b1111_1111_1111
    /// which calculate low 12 bits of physical address
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }

    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }
}

impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> VirtPageNum {
        VirtPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }

    pub fn offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }

    pub fn aligned(&self) -> bool {
        self.offset() == 0
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(pa: PhysAddr) -> Self {
        assert_eq!(pa.page_offset(), 0); // pa must be aligned to 4K
        pa.floor()
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(ppn: PhysPageNum) -> Self {
        Self(ppn.0 << PAGE_OFFSET)
    }
}

impl From<VirtPageNum> for VirtAddr {
    fn from(vpn: VirtPageNum) -> Self {
        Self(vpn.0 << PAGE_OFFSET)
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from(va: VirtAddr) -> Self {
        assert!(va.aligned());
        va.floor()
    }
}

impl PhysPageNum {
    pub fn pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, 512) }
    }

    pub fn bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysAddr = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut u8, 4096) }
    }

    pub fn as_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddr = (*self).into();
        unsafe { (pa.0 as *mut T).as_mut().unwrap() }
    }
}

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut idx = [0usize; 3];
        for i in (0..3).rev() {
            idx[i] = vpn & ((1 << 9) - 1);
            vpn >>= 9;
        }
        idx
    }
}

pub trait StepByOne {
    fn step(&mut self);
}

impl StepByOne for VirtPageNum {
    fn step(&mut self) {
        self.0 += 1;
    }
}

#[derive(Clone, Copy)]
pub struct SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    l: T,
    r: T,
}

impl<T> SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(l: T, r: T) -> Self {
        assert!(l <= r, "start {:?} > end {:?}", l, r);
        Self { l, r }
    }

    pub fn start(&self) -> T {
        self.l
    }

    pub fn end(&self) -> T {
        self.r
    }
}

impl<T> IntoIterator for SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;

    type IntoIter = SimpleRangeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleRangeIterator::new(self.l, self.r)
    }
}

pub struct SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    cur: T,
    end: T,
}

impl<T> SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(cur: T, end: T) -> Self {
        Self { cur, end }
    }
}

impl<T> Iterator for SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            None
        } else {
            let t = self.cur;
            self.cur.step();
            Some(t)
        }
    }
}

pub type VPNRange = SimpleRange<VirtPageNum>;
