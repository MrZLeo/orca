use alloc::sync::Arc;
use spin::Mutex;

use crate::{
    block_cache::get_block_cache,
    block_dev::BlockDevice,
    efs::EasyFileSystem,
    layout::{DirEntry, DiskInode, DiskInodeType, DIR_ENTRY_SIZE},
};

pub struct Inode {
    block_id: usize,
    block_offset: usize,
    block_dev: Arc<dyn BlockDevice>,
    fs: Arc<Mutex<EasyFileSystem>>,
}

impl Inode {
    pub fn new(
        block_id: usize,
        block_offset: usize,
        block_dev: Arc<dyn BlockDevice>,
        fs: Arc<Mutex<EasyFileSystem>>,
    ) -> Self {
        Self {
            block_id,
            block_offset,
            block_dev,
            fs,
        }
    }

    pub fn new_root(fs: Arc<Mutex<EasyFileSystem>>) -> Self {
        let block_dev = Arc::clone(&fs.lock().block_dev);
        let (block_id, block_offset) = fs.lock().get_disk_inode_pos(0);

        Self {
            block_id: block_id as usize,
            block_offset,
            block_dev,
            fs,
        }
    }

    pub fn read<V>(&self, f: impl FnOnce(&DiskInode) -> V) -> V {
        get_block_cache(self.block_id, Arc::clone(&self.block_dev))
            .lock()
            .read(self.block_offset, f)
    }

    pub fn modify<V>(&self, f: impl FnOnce(&mut DiskInode) -> V) -> V {
        get_block_cache(self.block_id, Arc::clone(&self.block_dev))
            .lock()
            .modify(self.block_offset, f)
    }

    pub fn find(&self, name: &str) -> Option<Arc<Inode>> {
        let fs = self.fs.lock();
        self.read(|inode| {
            self.find_inode_id(name, inode).map(|inode_id| {
                let (block_id, block_offset) = fs.get_disk_inode_pos(inode_id);
                Arc::new(Self::new(
                    block_id as usize,
                    block_offset,
                    self.block_dev.clone(),
                    self.fs.clone(),
                ))
            })
        })
    }

    pub fn find_inode_id(&self, name: &str, disk_inode: &DiskInode) -> Option<u32> {
        assert!(disk_inode.is_dir());
        let file_size = (disk_inode.size as usize) / DIR_ENTRY_SIZE;
        let mut dirent = DirEntry::new_empty();
        for i in 0..file_size {
            assert_eq!(
                disk_inode.read_at(i * DIR_ENTRY_SIZE, dirent.as_bytes_mut(), &self.block_dev),
                DIR_ENTRY_SIZE
            );
            if dirent.name() == name {
                return Some(dirent.inode());
            }
        }
        None
    }

    pub fn ls(&self) -> Vec<String> {
        let _fs = self.fs.lock();
        self.read(|inode| {
            let file_size = (inode.size as usize) / DIR_ENTRY_SIZE;
            let mut v = Vec::new();
            for i in 0..file_size {
                let mut dirent = DirEntry::new_empty();
                assert_eq!(
                    inode.read_at(i * DIR_ENTRY_SIZE, dirent.as_bytes_mut(), &self.block_dev),
                    DIR_ENTRY_SIZE
                );
                v.push(dirent.name().to_string())
            }
            v
        })
    }

    pub fn create(&self, name: &str) -> Option<Arc<Inode>> {
        let mut fs = self.fs.lock();

        let f = |root_inode: &DiskInode| {
            assert!(root_inode.is_dir());
            self.find_inode_id(name, root_inode)
        };
        if self.read(f).is_some() {
            return None;
        }

        let new_inode = fs.alloc_inode();
        let (block_id, block_offset) = fs.get_disk_inode_pos(new_inode);
        get_block_cache(block_id as usize, Arc::clone(&self.block_dev))
            .lock()
            .modify(block_offset, |disk_inode: &mut DiskInode| {
                disk_inode.init(DiskInodeType::File)
            });

        self.modify(|root_inode| {
            let file_size = (root_inode.size as usize) / DIR_ENTRY_SIZE;
            let new_size = (file_size + 1) * DIR_ENTRY_SIZE;

            self.increase_size(new_size as u32, root_inode, &mut fs);
            let dirent = DirEntry::new(name, new_inode);
            root_inode.write_at(
                file_size * DIR_ENTRY_SIZE,
                dirent.as_bytes(),
                &self.block_dev,
            );
        });

        let (block_id, block_offset) = fs.get_disk_inode_pos(new_inode);
        Some(Arc::new(Inode::new(
            block_id as usize,
            block_offset,
            self.block_dev.clone(),
            self.fs.clone(),
        )))
    }

    pub fn clear(&self) {
        let fs = self.fs.lock();
        self.modify(|disk_inode| {
            let data_block_dealloc = disk_inode.clear(&self.block_dev);
            assert_eq!(
                data_block_dealloc.len(),
                DiskInode::total_blocks(disk_inode) as usize
            );
            for data_block in data_block_dealloc {
                fs.dealloc_data(data_block)
            }
        })
    }

    pub fn read_at(&self, offset: usize, buf: &mut [u8]) -> usize {
        let _fs = self.fs.lock();
        self.read(|disk_inode| disk_inode.read_at(offset, buf, &self.block_dev))
    }

    pub fn write_at(&self, offset: usize, buf: &[u8]) -> usize {
        let mut fs = self.fs.lock();
        self.modify(|disk_inode| {
            self.increase_size((offset + buf.len()) as u32, disk_inode, &mut fs);
            disk_inode.write_at(offset, buf, &self.block_dev)
        })
    }

    fn increase_size(&self, new_size: u32, disk_inode: &mut DiskInode, fs: &mut EasyFileSystem) {
        if new_size < disk_inode.size {
            return;
        }
        let block_needed = disk_inode.block_needed(new_size);
        let mut v = Vec::new();
        for _ in 0..block_needed {
            v.push(fs.alloc_data());
        }
        disk_inode.increase(new_size, v, &self.block_dev);
    }
}
