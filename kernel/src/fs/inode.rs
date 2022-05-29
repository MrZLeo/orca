use crate::drivers::block::BLOCK_DEVICE;
use alloc::{sync::Arc, vec::Vec};
use easy_fs::{vfs::Inode, EasyFileSystem};
use spin::Mutex;

use super::File;

pub struct OSInode {
    readable: bool,
    writeable: bool,
    inner: Mutex<OSInodeInner>,
}

pub struct OSInodeInner {
    offset: usize,
    inode: Arc<Inode>,
}

impl OSInode {
    pub fn new(readable: bool, writeable: bool, inode: Arc<Inode>) -> Self {
        Self {
            readable,
            writeable,
            inner: Mutex::new(OSInodeInner { offset: 0, inode }),
        }
    }

    pub fn read_all(&self) -> Vec<u8> {
        let mut inner = self.inner.lock();
        let mut buffer = [0u8; 512];
        let mut res = Vec::new();
        loop {
            let len = inner.inode.read_at(inner.offset, &mut buffer);
            if len == 0 {
                return res;
            }
            inner.offset += len;
            res.extend_from_slice(&buffer[..len]);
        }

        res
    }
}

impl File for OSInode {
    fn read(&self, mut buf: crate::mm::page_table::UserBuf) -> usize {
        let mut inner = self.inner.lock();
        let mut total_size = 0;
        for slice in buf.buffers.iter_mut() {
            let read_size = inner.inode.read_at(inner.offset, *slice);
            if read_size == 0 {
                break;
            }
            inner.offset += read_size;
            total_size += read_size;
        }

        total_size
    }

    fn write(&self, buf: crate::mm::page_table::UserBuf) -> usize {
        let mut inner = self.inner.lock();
        let mut total_size = 0;
        for slice in buf.buffers.iter() {
            let write_size = inner.inode.write_at(inner.offset, *slice);

            // success write size equals to slice size
            assert_eq!(write_size, slice.len());

            inner.offset += write_size;
            total_size += write_size;
        }

        total_size
    }

    fn readable(&self) -> bool {
        self.readable
    }

    fn writeable(&self) -> bool {
        self.writeable
    }
}

lazy_static! {
    pub static ref ROOT_INODE: Arc<Inode> = {
        let efs = EasyFileSystem::open(BLOCK_DEVICE.clone());
        Arc::new(Inode::new_root(Arc::new(spin::Mutex::new(efs))))
    };
}

pub fn list_apps() {
    println!("---------- APPS ----------");
    for app in ROOT_INODE.ls() {
        println!("{}", app);
    }
    println!("---------- END  ----------");
}

bitflags! {
    /// flags to control the permission of file
    pub struct OpenFlags: u32 {
        const RDONLY = 0;
        const WRONLY = 1 << 0;
        const RDWR = 1 << 1;
        const CREATE = 1 << 9;
        const TRUNC = 1 << 10;
    }
}

impl OpenFlags {
    /// read_write: check permission
    /// @return: (is_able_to_read, is_able_to_write)
    pub fn read_write(&self) -> (bool, bool) {
        if self.is_empty() {
            (true, false)
        } else if self.contains(Self::WRONLY) {
            (false, true)
        } else {
            (true, true)
        }
    }
}

pub fn open_file(name: &str, flags: OpenFlags) -> Option<Arc<OSInode>> {
    let (readable, writeable) = flags.read_write();

    // if we are ask to create a new file
    if (flags.contains(OpenFlags::CREATE)) {
        // if we can find the file,
        // we have to clean it and return a new file
        if let Some(inode) = ROOT_INODE.find(name) {
            // clear size
            inode.clear();
            Some(Arc::new(OSInode::new(readable, writeable, inode)))
        } else {
            // if we don't find the file
            ROOT_INODE
                .create(name)
                .map(|inode| Arc::new(OSInode::new(readable, writeable, inode)))
        }
    } else {
        // if we should not create a new file
        ROOT_INODE.find(name).map(|inode| {
            if flags.contains(OpenFlags::TRUNC) {
                inode.clear();
            }
            Arc::new(OSInode::new(readable, writeable, inode))
        })
    }
}
