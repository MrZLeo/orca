use crate::block_dev::BlockDevice;
use crate::BLOCK_SIZE;
use alloc::sync::Arc;
use lru::LruCache;
use spin::Mutex;

pub struct BlockCache {
    cache: [u8; BLOCK_SIZE],
    block_id: usize,
    block_dev: Arc<dyn BlockDevice>,
    modified: bool,
}

impl BlockCache {
    pub fn new(block_id: usize, block_dev: Arc<dyn BlockDevice>) -> Self {
        let mut cache = [0u8; BLOCK_SIZE];
        block_dev.read_block(block_id, &mut cache);
        Self {
            cache,
            block_id,
            block_dev,
            modified: false,
        }
    }

    fn addr_of_offset(&self, offset: usize) -> usize {
        &self.cache[offset] as *const _ as usize
    }

    pub fn as_ref<T>(&self, offset: usize) -> &T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(type_size + offset <= BLOCK_SIZE);
        let addr = self.addr_of_offset(offset);
        unsafe { &*(addr as *const T) }
    }

    pub fn as_mut<T>(&mut self, offset: usize) -> &mut T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(type_size + offset <= BLOCK_SIZE);
        self.modified = true;
        let addr = self.addr_of_offset(offset);
        unsafe { &mut *(addr as *mut T) }
    }

    pub fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            self.block_dev.write_block(self.block_id, &self.cache);
        }
    }

    pub fn read<T, V>(&mut self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        f(self.as_ref(offset))
    }

    pub fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V {
        f(self.as_mut(offset))
    }
}

impl Drop for BlockCache {
    fn drop(&mut self) {
        self.sync();
    }
}

const BLOCK_CACHE_SIZE: usize = 16;

pub struct BlockCacheManager {
    cache: LruCache<usize, Arc<Mutex<BlockCache>>>,
}

impl BlockCacheManager {
    pub fn new() -> Self {
        Self {
            cache: LruCache::new(BLOCK_CACHE_SIZE),
        }
    }

    pub fn get(
        &mut self,
        block_id: usize,
        block_dev: Arc<dyn BlockDevice>,
    ) -> Arc<Mutex<BlockCache>> {
        if let Some(cache) = self.cache.get(&block_id) {
            Arc::clone(cache)
        } else {
            // can not find cache,
            // get data from disk and put it into cache
            let new_cache = Arc::new(Mutex::new(BlockCache::new(
                block_id,
                Arc::clone(&block_dev),
            )));
            self.cache.put(block_id, Arc::clone(&new_cache));
            new_cache
        }
    }
}

lazy_static! {
    pub static ref BLOCK_CACHE_MANAGER: Mutex<BlockCacheManager> =
        Mutex::new(BlockCacheManager::new());
}

pub fn get_block_cache(block_id: usize, block_dev: Arc<dyn BlockDevice>) -> Arc<Mutex<BlockCache>> {
    BLOCK_CACHE_MANAGER.lock().get(block_id, block_dev)
}

pub fn block_cache_sync_all() {
    let manager = BLOCK_CACHE_MANAGER.lock();
    // for (_, cache) in manager.queue.iter() {
    //     cache.lock().sync();
    // }
    for (_, cache) in manager.cache.iter() {
        cache.lock().sync();
    }
}
