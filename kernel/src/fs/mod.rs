use crate::mm::page_table::UserBuf;

pub mod inode;
pub mod stdio;

pub trait File: Send + Sync {
    fn read(&self, buf: UserBuf) -> usize;
    fn write(&self, buf: UserBuf) -> usize;
    fn readable(&self) -> bool;
    fn writeable(&self) -> bool;
}
