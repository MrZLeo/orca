pub trait File: Send + Sync {
    fn read(&self, buf: UserBuf) -> usize;
    fn write(&self, buf: UserBuf) -> usize;
}
