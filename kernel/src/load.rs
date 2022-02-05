use crate::config::*;
use crate::trap::TrapContext;
use core::arch::asm;

extern "C" {
    fn _num_app();
}

#[inline]
fn apps_ptr() -> *const usize {
    (_num_app as usize) as *const usize
}

#[inline]
fn num_app() -> usize {
    unsafe { apps_ptr().read_volatile() }
}

#[inline]
fn base_i(app_id: usize) -> usize {
    APP_BASE_ADDR + app_id * APP_SIZE_LIMIT
}

fn load_app() {
    let apps_ptr = apps_ptr();
    let num_app = num_app();
    let app_start = unsafe { core::slice::from_raw_parts(apps_ptr.add(1), num_app + 1) };

    unsafe {
        asm!("fence.i"); // clear instruction cache
    }

    for i in 0..num_app {
        let base_i = base_i(i);
        (base_i..base_i + APP_SIZE_LIMIT)
            .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });

        let src = unsafe {
            core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
        };

        let dst = unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };

        dst.copy_from_slice(src);
    }
}

#[repr(align(4096))]
struct Stack {
    data: [u8; KERNEL_STACK_SIZE],
}

static KERNEL_STACK: Stack = Stack {
    data: [0; KERNEL_STACK_SIZE],
};

static USER_STACK: Stack = Stack {
    data: [0; USER_STACK_SIZE],
};

impl Stack {
    pub fn sp(&self) -> usize {
        self.data.as_ptr() as usize + self.data.len()
    }

    pub fn push_ctx(&self, ctx: TrapContext) -> &'static mut TrapContext {
        let ctx_ptr = (self.sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *ctx_ptr = ctx;
            ctx_ptr.as_mut().unwrap()
        }
    }
}
