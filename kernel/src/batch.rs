/// AppManager:
/// - works in batch system
/// - store info of apps
struct AppManager {
    cur_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
    len: usize,
}

use crate::lang_item::panic;
use crate::sync::UniProcSafeCell;
use crate::trap::Context;
use core::arch::asm;
use lazy_static::lazy_static;

/// const about AppManager
const MAX_APP_NUM: usize = 16; // max size of app is 16
const APP_BASE_ADDR: usize = 0x8040_0000;
const APP_SIZE_LIMIT: usize = 0x2_0000;

lazy_static! {
    static ref APP_MANAGER: UniProcSafeCell<AppManager> = unsafe {
        UniProcSafeCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = (_num_app as usize) as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                cur_app: 0,
                app_start,
                len: num_app,
            }
        })
    };
}

impl AppManager {
    pub fn app_info(&self) {
        info!("[kernel] num_app={}", self.len);
        for i in 0..self.len {
            info!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    pub fn cur_app(&self) -> usize {
        self.cur_app
    }

    pub fn next_app(&mut self) {
        self.cur_app += 1;
    }

    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.len {
            error!("All tasks done");
        }

        info!("[kernel] Loading app_{}", app_id);

        // clear instruction cache
        asm!("fence.i");

        // clear memory
        core::slice::from_raw_parts_mut(APP_BASE_ADDR as *mut u8, APP_SIZE_LIMIT).fill(0);

        let src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );

        let mut dst = core::slice::from_raw_parts_mut(APP_BASE_ADDR as *mut u8, src.len());

        dst.copy_from_slice(src);
    }
}

pub fn batch_init() {
    print_batch_info();
}

pub fn print_batch_info() {
    APP_MANAGER.borrow_mut().app_info();
}

// stack for batch-os
const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

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

    pub fn push_ctx(&self, ctx: Context) -> &'static mut Context {
        let ctx_ptr = (self.sp() - core::mem::size_of::<Context>()) as *mut Context;
        unsafe {
            *ctx_ptr = ctx;
            ctx_ptr.as_mut().unwrap()
        }
    }
}

pub fn batch_schedule() -> ! {
    let mut app_manager = APP_MANAGER.borrow_mut();
    let cur_app = app_manager.cur_app();

    unsafe {
        app_manager.load_app(cur_app);
    }

    app_manager.next_app();
    drop(app_manager);

    extern "C" {
        fn __restore(ctx: usize);
    }

    unsafe {
        __restore(
            KERNEL_STACK.push_ctx(Context::app_init_cxt(APP_BASE_ADDR, USER_STACK.sp())) as *const _
                as usize,
        );
    }

    panic!("Unreachable: batch schedule");
}
