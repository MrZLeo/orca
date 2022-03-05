use alloc::vec::Vec;

use crate::sync::UniProcSafeCell;

pub struct Pid(pub usize);

struct PidAllocator {
    cur: usize,
    recycled: Vec<usize>,
}

impl PidAllocator {
    pub fn new() -> Self {
        Self {
            cur: 0,
            recycled: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> Pid {
        if let Some(pid) = self.recycled.pop() {
            Pid(pid)
        } else {
            self.cur += 1;
            Pid(self.cur - 1)
        }
    }

    pub fn dealloc(&mut self, pid: usize) {
        // pid is alloced befor dealloc
        assert!(self.cur > pid);

        // avoid double free
        assert!(
            self.recycled.iter().any(|&x| x == pid),
            "pid has been dealloc"
        );

        self.recycled.push(pid);
    }
}

impl Drop for Pid {
    fn drop(&mut self) {
        PID_ALLOCATOR.borrow_mut().dealloc(self.0);
    }
}

lazy_static! {
    static ref PID_ALLOCATOR: UniProcSafeCell<PidAllocator> =
        UniProcSafeCell::new(PidAllocator::new());
}

pub fn pid_alloc() -> Pid {
    PID_ALLOCATOR.borrow_mut().alloc()
}
