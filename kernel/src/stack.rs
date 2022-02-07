use crate::config::*;
use crate::trap::TrapContext;

/// temporary implementation of Stack
/// `USER_STACK` will have to use same capacity as `KERNEL_STACK`
#[derive(Clone, Copy)]
#[repr(align(4096))]
pub struct Stack {
    data: [u8; KERNEL_STACK_SIZE],
}

pub static KERNEL_STACK: [Stack; MAX_APP_NUM] = [Stack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];

pub static USER_STACK: [Stack; MAX_APP_NUM] = [Stack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

impl Stack {
    pub fn sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_ctx(&self, cxt: TrapContext) -> usize {
        let cxt_ptr = (self.sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cxt_ptr = cxt;
        }
        cxt_ptr as usize
    }
}
