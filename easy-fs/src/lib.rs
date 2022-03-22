pub mod bitmap;
pub mod block_cache;
pub mod block_dev;
pub mod efs;
pub mod layout;
pub mod vfs;

extern crate alloc;
extern crate lru;

#[macro_use]
extern crate lazy_static;

pub const BLOCK_SIZE: usize = 512;
pub use block_dev::BlockDevice;
pub use efs::EasyFileSystem;
