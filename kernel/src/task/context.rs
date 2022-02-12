use crate::trap::trap_return;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn new(ra: usize, sp: usize, s: [usize; 12]) -> Self {
        Self { ra, sp, s }
    }

    pub fn from_zero() -> Self {
        Self::new(0, 0, [0; 12])
    }

    pub fn with_trap_return(sp: usize) -> Self {
        Self::new(trap_return as usize, sp, [0; 12])
    }

    pub fn sp(&self) -> usize {
        self.sp
    }
}
