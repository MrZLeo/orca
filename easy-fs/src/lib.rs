mod block_cache;
mod block_dev;

extern crate alloc;
extern crate lru;

#[macro_use]
extern crate lazy_static;

pub const BLOCK_SIZE: usize = 512;