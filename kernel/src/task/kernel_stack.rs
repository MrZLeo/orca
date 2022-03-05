use super::pid::Pid;
use crate::{
    config::{KERNEL_STACK_SIZE, PAGE_SIZE, TRAMPOLINE},
    mm::{
        address::VirtAddr,
        memory_set::{MapPermission, KERNEL_SPACE},
    },
};
pub struct KernelStack {
    pid: Pid,
    top: usize,
}

impl KernelStack {
    pub fn new(pid: Pid) -> Self {
        let (bottom, top) = kernel_stack_position(pid.0);
        KERNEL_SPACE.borrow_mut().insert_framed_area(
            bottom.into(),
            top.into(),
            MapPermission::R | MapPermission::W,
        );
        Self { pid, top }
    }

    pub fn top(&self) -> usize {
        self.top
    }

    pub fn push<T>(&mut self, v: T) -> *mut T
    where
        T: Sized,
    {
        let ptr = (self.top - core::mem::size_of::<T>()) as *mut T;
        unsafe {
            *ptr = v;
        }
        self.top -= core::mem::size_of::<T>();
        ptr
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        let (bottom, _) = kernel_stack_position(self.pid.0);
        let bottom_va: VirtAddr = bottom.into();
        KERNEL_SPACE.borrow_mut().remove(bottom_va.into());
    }
}

pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}
