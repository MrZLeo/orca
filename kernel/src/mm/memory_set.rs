use core::arch::asm;

use crate::{
    config::{MEMORY_END, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT, USER_STACK_SIZE},
    sync::UniProcSafeCell,
};
use alloc::{collections::BTreeMap, sync::Arc, vec::Vec};
use lazy_static::lazy_static;
use riscv::register::satp;

use super::{
    address::{PhysAddr, PhysPageNum, StepByOne, VPNRange, VirtAddr, VirtPageNum},
    frame_allocator::{frame_alloc, FrameTracker},
    page_table::{PTEFlags, PageTable, PageTableEntry},
};

// import position of differnet sections
use crate::config::ebss;
use crate::config::edata;
use crate::config::ekernel;
use crate::config::erodata;
use crate::config::etext;
use crate::config::sbss_with_stack;
use crate::config::sdata;
use crate::config::srodata;
use crate::config::stext;
use crate::config::strampoline;

pub struct MapArea {
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

impl MapArea {
    pub fn new(
        va_start: VirtAddr,
        va_end: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission,
    ) -> Self {
        let va_start: VirtPageNum = va_start.floor();
        let va_end: VirtPageNum = va_end.ceil();
        Self {
            vpn_range: VPNRange::new(va_start, va_end),
            data_frames: BTreeMap::new(),
            map_type,
            map_perm,
        }
    }

    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(page_table, vpn);
        }
    }

    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one(page_table, vpn);
        }
    }

    pub fn copy_data(&self, page_table: &mut PageTable, data: &[u8]) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut cur_vpn = self.vpn_range.start();
        let len = data.len();
        loop {
            let src = &data[start..len.min(start + PAGE_SIZE)];
            let dst = &mut page_table.translate(cur_vpn).unwrap().ppn().bytes_array()[..src.len()];
            dst.copy_from_slice(src);
            start += PAGE_SIZE;
            if start >= len {
                break;
            }
            cur_vpn.step();
        }
    }

    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn, frame);
            }
            MapType::Identical => ppn = PhysPageNum(vpn.0),
        }

        let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }

    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if MapType::Framed == self.map_type {
            self.data_frames.remove(&vpn);
        }

        page_table.unmap(vpn);
    }

    pub fn from_another(area: &MapArea) -> Self {
        Self {
            vpn_range: VPNRange::new(area.vpn_range.start(), area.vpn_range.end()),
            data_frames: BTreeMap::new(),
            map_type: area.map_type,
            map_perm: area.map_perm,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MapType {
    Identical,
    Framed,
}

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MemorySet {
    pub fn new() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    pub fn token(&self) -> usize {
        self.page_table.token()
    }

    fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut self.page_table);
        if let Some(data) = data {
            map_area.copy_data(&mut self.page_table, data);
        }
        self.areas.push(map_area);
    }

    /// Must assume there is no conflict
    pub fn insert_framed_area(
        &mut self,
        start_va: VirtAddr,
        end_va: VirtAddr,
        permission: MapPermission,
    ) {
        self.push(
            MapArea::new(start_va, end_va, MapType::Framed, permission),
            None,
        )
    }

    pub fn new_kernel() -> Self {
        let mut memory_set = Self::new();
        memory_set.map_trampoline();

        kernel!("mapping .text section");
        memory_set.push(
            MapArea::new(
                (stext as usize).into(),
                (etext as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
            ),
            None,
        );

        kernel!("mapping .rodata section");
        memory_set.push(
            MapArea::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Identical,
                MapPermission::R,
            ),
            None,
        );

        kernel!("mapping .data section");
        memory_set.push(
            MapArea::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        kernel!("mapping .bss section");
        memory_set.push(
            MapArea::new(
                (sbss_with_stack as usize).into(),
                (ebss as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        kernel!("mapping physical memory");
        memory_set.push(
            MapArea::new(
                (ekernel as usize).into(),
                MEMORY_END.into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        memory_set
    }

    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        let mut memory_set = Self::new();
        memory_set.map_trampoline();

        // mapping program header with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf"); // magic: ELF
        let ph_count = elf_header.pt2.ph_count();
        let mut max_end_vpn = VirtPageNum(0);
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let va_start: VirtAddr = (ph.virtual_addr() as usize).into();
                let va_end: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                let mut map_perm = MapPermission::U;
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    map_perm |= MapPermission::R;
                }
                if ph_flags.is_write() {
                    map_perm |= MapPermission::W;
                }
                if ph_flags.is_execute() {
                    map_perm |= MapPermission::X;
                }
                let map_area = MapArea::new(va_start, va_end, MapType::Framed, map_perm);
                max_end_vpn = map_area.vpn_range.end();
                memory_set.push(
                    map_area,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                )
            }
        }

        // mapping user stack with U flag
        let max_end_va: VirtAddr = max_end_vpn.into();
        let mut user_stack_bottom: usize = max_end_va.into();

        // guard page
        user_stack_bottom += PAGE_SIZE;
        let user_stack_top = user_stack_bottom + USER_STACK_SIZE;
        memory_set.push(
            MapArea::new(
                user_stack_bottom.into(),
                user_stack_top.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W | MapPermission::U,
            ),
            None,
        );

        // map TrapContext
        memory_set.push(
            MapArea::new(
                TRAP_CONTEXT.into(),
                TRAMPOLINE.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        (
            memory_set,
            user_stack_top,
            elf.header.pt2.entry_point() as usize,
        )
    }

    fn map_trampoline(&mut self) {
        self.page_table.map(
            VirtAddr::from(TRAMPOLINE).into(),
            PhysAddr::from(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X,
        )
    }

    pub fn activate(&self) {
        debug!("start activate virtual memory");
        let satp = self.token();
        debug!("get token: {:x}", satp);
        unsafe {
            // activate virtual memory
            satp::write(satp);
            // clear TLB
            asm!("sfence.vma");
        }
        debug!("activate virtual memory");
    }

    pub fn page_table(&self) -> &PageTable {
        &self.page_table
    }

    pub fn page_table_mut(&mut self) -> &mut PageTable {
        &mut self.page_table
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.page_table.translate(vpn)
    }

    pub fn remove(&mut self, vpn: VirtPageNum) {
        if let Some((idx, area)) = self
            .areas
            .iter_mut()
            .enumerate()
            .find(|(idx, a)| a.vpn_range.start() == vpn)
        {
            area.unmap(&mut self.page_table);
            self.areas.remove(idx);
        }
    }

    pub fn from_exited(mmset: &MemorySet) -> Self {
        let mut memory_set = MemorySet::new();
        memory_set.map_trampoline();

        mmset.areas.iter().for_each(|area| {
            let new_area = MapArea::from_another(area);
            memory_set.push(new_area, None);
            for vpn in area.vpn_range {
                let src = mmset.translate(vpn).unwrap().ppn();
                let des = memory_set.translate(vpn).unwrap().ppn();
                des.bytes_array().copy_from_slice(src.bytes_array());
            }
        });

        memory_set
    }

    pub fn recycle_pages(&mut self) {
        self.areas.clear();
    }
}

lazy_static! {
    pub static ref KERNEL_SPACE: Arc<UniProcSafeCell<MemorySet>> =
        Arc::new(UniProcSafeCell::new(MemorySet::new_kernel()));
}
