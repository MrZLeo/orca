#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(unused)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod config;
mod lang_item;
mod loader;
mod mm;
mod orca_logo;
mod sbi;
mod sync;
mod syscall;
mod task;
mod test;
mod timer;
mod trap;

#[macro_use]
extern crate bitflags;

use core::arch::global_asm;

extern crate alloc;

global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn __main() {
    clear_bss();
    sys_info();
    kernel!("Hello World!");

    mm::init();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_strigger();
    #[cfg(feature = "kernel_test")]
    {
        test::main();
    }

    debug!("syscall test");
    let t = syscall::sys_time();
    debug!("t: {}", t);

    task::start();
    panic!("unreachable: __main ended");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    // (sbss as usize..ebss as usize).for_each(|x| unsafe { (x as *mut u8).write_volatile(0) });
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0)
    }
}

fn sys_info() {
    extern "C" {
        fn skernel();
        fn ekernel();
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss_with_stack();
        fn ebss();
    }
    print!("\x1b[1m{}\x1b[0m", orca_logo::ORCA_LOGO);
    info!(
        "kernel range [{:#x}, {:#x}]",
        skernel as usize, ekernel as usize
    );
    info!(".text [{:#x}, {:#x}]", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x}]", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x}]", sdata as usize, edata as usize);
    info!(
        ".bss [{:#x}, {:#x}]",
        sbss_with_stack as usize, ebss as usize
    );
}
