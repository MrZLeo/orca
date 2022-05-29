use alloc::sync::Arc;
use easy_fs::BlockDevice;

mod virtio_blk;

type BlockDeviceImpl = virtio_blk::VirtIOBlock;

lazy_static! {
    pub static ref BLOCK_DEVICE: Arc<dyn BlockDevice> = Arc::new(BlockDeviceImpl::new());
}
