use crate::config::*;
use crate::stack::KERNEL_STACK;
use crate::stack::USER_STACK;
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
pub fn num_app() -> usize {
    unsafe { ((_num_app as usize) as *const usize).read_volatile() }
}

#[inline]
fn base(app_id: usize) -> usize {
    APP_BASE_ADDR + app_id * APP_SIZE_LIMIT
}

pub fn load_app() {
    let apps_ptr = (_num_app as usize) as *const usize;
    let num_app = num_app();
    let app_start = unsafe { core::slice::from_raw_parts(apps_ptr.add(1), num_app + 1) };

    unsafe {
        asm!("fence.i"); // clear instruction cache
    }

    for i in 0..num_app {
        let base_i = base(i);
        (base_i..base_i + APP_SIZE_LIMIT)
            .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });

        let src = unsafe {
            core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
        };

        let dst = unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };

        dst.copy_from_slice(src);
    }
}

pub fn init_cxt(app_id: usize) -> usize {
    debug!("app_id: {}", app_id);
    debug!("basei: {:x}", base(app_id));
    debug!("user stack sp: {:x}", USER_STACK[app_id].sp());
    KERNEL_STACK[app_id].push_ctx(TrapContext::app_init_cxt(
        base(app_id),
        USER_STACK[app_id].sp(),
    ))
}
