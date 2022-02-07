use core::cell::{RefCell, RefMut};

pub struct UniProcSafeCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UniProcSafeCell<T> {}

impl<T> UniProcSafeCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
