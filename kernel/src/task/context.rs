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

    pub fn with_restore(sp: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self::new(__restore as usize, sp, [0; 12])
    }

    pub fn sp(&self) -> usize {
        self.sp
    }
}
