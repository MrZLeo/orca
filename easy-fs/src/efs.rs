use crate::bitmap::{self, Bitmap};
use crate::block_cache::get_block_cache;
use crate::block_dev::BlockDevice;
use crate::layout::{DataBlock, DiskInode, DiskInodeType, SuperBlock, EFS_MAGIC};
use crate::BLOCK_SIZE;
use alloc::sync::Arc;

pub struct EasyFileSystem {
    pub block_dev: Arc<dyn BlockDevice>,
    pub inode_bitmap: Bitmap,
    pub data_bitmap: Bitmap,
    inode_start_block: u32,
    data_start_block: u32,
}

impl EasyFileSystem {
    pub fn new(
        block_dev: Arc<dyn BlockDevice>,
        total_blocks: u32,
        inode_bitmap_blocks: u32,
    ) -> Self {
        // inode memory allocation
        let inode_bitmap = Bitmap::new(1, inode_bitmap_blocks as usize);
        let inode_num = inode_bitmap.total();
        let inode_area_blocks =
            ((inode_num * core::mem::size_of::<DiskInode>() + BLOCK_SIZE - 1) / BLOCK_SIZE) as u32;
        let inode_total_block = inode_area_blocks + inode_bitmap_blocks;

        // data memory allocation
        let data_total_blocks = total_blocks - inode_total_block - 1; // 1 -> super block
        let data_bitmap_blocks =
            (data_total_blocks + bitmap::BLOCK_BITS as u32) / (bitmap::BLOCK_BITS as u32 + 1);
        let data_bitmap = Bitmap::new(1 + inode_total_block as usize, data_bitmap_blocks as usize);
        let data_area_blocks = data_total_blocks - data_bitmap_blocks;

        let efs = Self {
            block_dev: Arc::clone(&block_dev),
            inode_bitmap,
            data_bitmap,
            inode_start_block: 1 + inode_bitmap_blocks,
            data_start_block: 1 + inode_total_block + data_bitmap_blocks,
        };

        // clear
        for i in 0..total_blocks {
            get_block_cache(i as usize, Arc::clone(&block_dev))
                .lock()
                .modify(0, |block: &mut DataBlock| {
                    block.iter_mut().for_each(|x| *x = 0)
                });
        }

        // init super block
        get_block_cache(0, Arc::clone(&block_dev))
            .lock()
            .modify(0, |block: &mut SuperBlock| {
                block.init(
                    EFS_MAGIC,
                    total_blocks,
                    inode_bitmap_blocks,
                    inode_area_blocks,
                    data_bitmap_blocks,
                    data_area_blocks,
                );
            });

        // init root "/"
        assert_eq!(efs.alloc_inode(), 0, "first inode must be 0");
        let (root_inode_block, root_inode_offset) = efs.get_disk_inode_pos(0);
        get_block_cache(root_inode_block as usize, Arc::clone(&block_dev))
            .lock()
            .modify(root_inode_offset, |block: &mut DiskInode| {
                block.init(DiskInodeType::Dir)
            });

        efs
    }

    pub fn open(block_dev: Arc<dyn BlockDevice>) -> Self {
        get_block_cache(0, Arc::clone(&block_dev))
            .lock()
            .read(0, |super_block: &SuperBlock| {
                assert!(super_block.is_valid(), "Error: Unknown file format");
                let total_inode_block =
                    super_block.inode_bitmap_blocks + super_block.inode_area_blocks;
                let efs = Self {
                    block_dev,
                    inode_bitmap: Bitmap::new(1, super_block.inode_bitmap_blocks as usize),
                    data_bitmap: Bitmap::new(
                        1 + total_inode_block as usize,
                        super_block.data_bitmap_blocks as usize,
                    ),
                    inode_start_block: 1 + super_block.inode_bitmap_blocks,
                    data_start_block: 1 + total_inode_block + super_block.data_bitmap_blocks,
                };

                efs
            })
    }

    pub fn alloc_inode(&self) -> u32 {
        self.inode_bitmap
            .alloc(&Arc::clone(&self.block_dev))
            .unwrap() as u32
    }

    pub fn dealloc_inode(&self) {
        todo!()
    }

    pub fn get_disk_inode_pos(&self, inode_id: u32) -> (u32, usize) {
        let inode_size = core::mem::size_of::<DiskInode>();
        let inodes_per_block = (BLOCK_SIZE / inode_size) as u32;
        let block_id = self.inode_start_block + (inode_id / inodes_per_block);

        (
            block_id,
            (inode_id % inodes_per_block) as usize * inode_size,
        )
    }

    pub fn get_data_block_id(&self, data_block: u32) -> u32 {
        self.data_start_block + data_block
    }

    /// @return block-id
    pub fn alloc_data(&self) -> u32 {
        self.data_bitmap
            .alloc(&Arc::clone(&self.block_dev))
            .unwrap() as u32
            + self.data_start_block
    }

    pub fn dealloc_data(&self, block_id: u32) {
        get_block_cache(block_id as usize, Arc::clone(&self.block_dev))
            .lock()
            .modify(0, |block: &mut DataBlock| {
                block.iter_mut().for_each(|x| *x = 0)
            });
        self.data_bitmap
            .dealloc(&self.block_dev, (block_id - self.data_start_block) as usize)
    }
}
