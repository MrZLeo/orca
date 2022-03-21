use crate::{block_cache::get_block_cache, block_dev::BlockDevice, BLOCK_SIZE};
use alloc::sync::Arc;

pub const EFS_MAGIC: u32 = 0x3b800001;

#[repr(C)]
pub struct SuperBlock {
    magic: u32,
    pub total_blocks: u32,
    pub inode_bitmap_blocks: u32,
    pub inode_area_blocks: u32,
    pub data_bitmap_blocks: u32,
    pub data_area_blocks: u32,
}

impl SuperBlock {
    pub fn init(
        &mut self,
        magic: u32,
        total_blocks: u32,
        inode_bitmap_blocks: u32,
        inode_area_blocks: u32,
        data_bitmap_blocks: u32,
        data_area_blocks: u32,
    ) {
        *self = Self {
            magic,
            total_blocks,
            inode_bitmap_blocks,
            inode_area_blocks,
            data_bitmap_blocks,
            data_area_blocks,
        };
    }

    pub fn is_valid(&self) -> bool {
        self.magic == EFS_MAGIC
    }
}

const INODE_DIRECT_NUM: usize = 28;
const INODE_INDIRECT_NUM: usize = BLOCK_SIZE / 4; // 1 block devided by u32 (4 bytes)
const INDIRECT1_BOUND: usize = INODE_DIRECT_NUM + INODE_INDIRECT_NUM;

#[repr(C)]
pub struct DiskInode {
    pub size: u32,
    pub direct: [u32; INODE_DIRECT_NUM],
    pub indirect1: u32,
    pub indirect2: u32,
    _type: DiskInodeType,
}

#[derive(PartialEq)]
pub enum DiskInodeType {
    File,
    Dir,
}

type IndirectBlock = [u32; BLOCK_SIZE / 4];
pub type DataBlock = [u8; BLOCK_SIZE];

impl DiskInode {
    pub fn init(&mut self, _type: DiskInodeType) {
        self.size = 0;
        self.direct.iter_mut().for_each(|x| *x = 0);
        self.indirect1 = 0;
        self.indirect2 = 0;
        self._type = _type;
    }

    pub fn is_file(&self) -> bool {
        self._type == DiskInodeType::File
    }

    pub fn is_dir(&self) -> bool {
        self._type == DiskInodeType::Dir
    }

    pub fn get_block_id(&self, inner_id: u32, block_dev: &Arc<dyn BlockDevice>) -> u32 {
        let inner_id = inner_id as usize;
        if inner_id <= INODE_DIRECT_NUM {
            self.direct[inner_id]
        } else if inner_id <= INDIRECT1_BOUND {
            get_block_cache(self.indirect1 as usize, Arc::clone(block_dev))
                .lock()
                .read(0, |indirect_block: &IndirectBlock| {
                    indirect_block[inner_id - INODE_DIRECT_NUM]
                })
        } else {
            let indirect1 = get_block_cache(self.indirect2 as usize, Arc::clone(block_dev))
                .lock()
                .read(0, |indirect_block: &IndirectBlock| {
                    indirect_block[(inner_id - INDIRECT1_BOUND) / INODE_INDIRECT_NUM]
                });

            get_block_cache(indirect1 as usize, Arc::clone(block_dev))
                .lock()
                .read(0, |indirect_block: &IndirectBlock| {
                    indirect_block[(inner_id - INDIRECT1_BOUND) % INODE_INDIRECT_NUM]
                })
        }
    }

    fn __data_blocks(size: u32) -> u32 {
        (size + BLOCK_SIZE as u32 - 1) / BLOCK_SIZE as u32
    }

    pub fn data_block(&self) -> u32 {
        Self::__data_blocks(self.size)
    }

    fn __total_blocks(size: u32) -> u32 {
        let data = Self::__data_blocks(size);
        if data <= INODE_DIRECT_NUM as u32 {
            data
        } else if data <= INODE_INDIRECT_NUM as u32 {
            data + 1
        } else {
            data + 2
                + (data - INDIRECT1_BOUND as u32 + INODE_INDIRECT_NUM as u32 - 1)
                    / INODE_INDIRECT_NUM as u32
        }
    }

    pub fn total_blocks(&self) -> u32 {
        Self::__total_blocks(self.size)
    }

    pub fn block_needed(&self, new_size: u32) -> u32 {
        assert!(new_size > self.size);
        Self::__total_blocks(new_size) - self.total_blocks()
    }

    pub fn increase(
        &mut self,
        new_size: u32,
        new_blocks: Vec<u32>,
        block_dev: &Arc<dyn BlockDevice>,
    ) {
    }

    /// @return `Vec<u32>`: the blocks that this file used
    pub fn clear(&mut self, block_dev: &Arc<dyn BlockDevice>) -> Vec<u32> {
        todo!()
    }

    pub fn read_at(
        &self,
        offset: usize,
        buf: &mut [u8],
        block_dev: &Arc<dyn BlockDevice>,
    ) -> usize {
        let mut start = offset;
        let end = (offset + buf.len()).min(self.size as usize);
        if start > end {
            return 0;
        }

        let mut start_bk = start / BLOCK_SIZE;
        let mut read_sz = 0usize;

        loop {
            let mut cur_block_end = (start_bk / BLOCK_SIZE + 1) * BLOCK_SIZE;
            cur_block_end = cur_block_end.min(end);
            let inner_read_size = cur_block_end - start;
            let dst = &mut buf[read_sz..read_sz + inner_read_size];
            get_block_cache(
                self.get_block_id(start_bk as u32, block_dev) as usize,
                Arc::clone(block_dev),
            )
            .lock()
            .read(0, |read_bk: &DataBlock| {
                let src = &read_bk[start % BLOCK_SIZE..start % BLOCK_SIZE + inner_read_size];
                dst.copy_from_slice(src);
            });
            read_sz += inner_read_size;
            if cur_block_end == end {
                break;
            }
            start_bk += 1;
            start = cur_block_end;
        }

        read_sz
    }

    pub fn write_at(
        &mut self,
        offset: usize,
        buf: &[u8],
        block_device: &Arc<dyn BlockDevice>,
    ) -> usize {
        todo!()
    }
}

const NAME_LEN_LIMIT: usize = 27;
pub const DIR_ENTRY_SIZE: usize = 32;

#[repr(C)]
pub struct DirEntry {
    name: [u8; NAME_LEN_LIMIT + 1],
    inode: u32,
}

impl DirEntry {
    pub fn new(name: &str, inode: u32) -> Self {
        let mut bytes = [0u8; NAME_LEN_LIMIT + 1];
        bytes[..name.len()].copy_from_slice(name.as_bytes());
        Self { name: bytes, inode }
    }

    pub fn new_empty() -> Self {
        Self {
            name: [0u8; NAME_LEN_LIMIT + 1],
            inode: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self as *const _ as usize as *const u8, DIR_ENTRY_SIZE)
        }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(self as *mut _ as usize as *mut u8, DIR_ENTRY_SIZE)
        }
    }

    pub fn inode(&self) -> u32 {
        self.inode
    }

    pub fn name(&self) -> &str {
        let len = (0usize..).find(|&i| self.name[i] == 0u8).unwrap();
        core::str::from_utf8(&self.name[..len]).unwrap()
    }
}
