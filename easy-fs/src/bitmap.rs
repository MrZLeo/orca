use crate::block_cache::get_block_cache;
use crate::block_dev::BlockDevice;
use crate::BLOCK_SIZE;
use alloc::sync::Arc;
use core::option::Option;
use core::option::Option::None;
use core::option::Option::Some;

type BitmapBlock = [u64; 64];
pub const BLOCK_BITS: usize = BLOCK_SIZE * 8;

pub struct Bitmap {
    start_block_id: usize,
    len: usize,
}

impl Bitmap {
    pub fn new(start_block_id: usize, len: usize) -> Self {
        Self {
            start_block_id,
            len,
        }
    }

    pub fn alloc(&self, block_dev: &Arc<dyn BlockDevice>) -> Option<usize> {
        for block_id in 0..self.len {
            let pos = get_block_cache(block_id + self.start_block_id, Arc::clone(block_dev))
                .lock()
                .modify(0, |bitmap_block: &mut BitmapBlock| {
                    if let Some((bits64_pos, inner_pos)) = bitmap_block
                        .iter()
                        .enumerate()
                        .find(|(_, bits64)| **bits64 != u64::MAX) // find a double word that contains at least one 0
                        .map(|(bits64_pos, bits64)| (bits64_pos, bits64.trailing_ones() as usize))
                    {
                        bitmap_block[bits64_pos] |= 1u64 << inner_pos;
                        Some(block_id * BLOCK_BITS + bits64_pos * 64 + inner_pos as usize)
                    } else {
                        None
                    }
                });
            if pos.is_some() {
                return pos;
            }
        }
        None
    }

    /// @return `(block_pos, bits64_pos, inner_pos)`
    fn decomposition(bit: usize) -> (usize, usize, usize) {
        let block_pos = bit / BLOCK_BITS;
        let bit = bit % BLOCK_BITS;
        (block_pos, bit / 64, bit % 64)
    }

    pub fn dealloc(&self, block_dev: &Arc<dyn BlockDevice>, bit: usize) {
        let (block_pos, bits64_pos, inner_pos) = Bitmap::decomposition(bit);
        get_block_cache(self.start_block_id + block_pos, Arc::clone(block_dev))
            .lock()
            .modify(0, |bitmap_block: &mut BitmapBlock| {
                assert!(
                    bitmap_block[bits64_pos] & (1u64 << inner_pos) > 0,
                    "bitmap must be alloced before dealloc"
                );
                bitmap_block[bits64_pos] &= !(1u64 << inner_pos);
            });
    }

    /// @return total_blocks that `Bitmap` can represent
    pub fn total(&self) -> usize {
        self.len * BLOCK_BITS
    }
}
