#![no_std]
#![feature(linkage)]
#![allow(unused)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod lang_item;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("can not find main()");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|x| unsafe { (x as *mut u8).write_volatile(0) });
}

/// syscall for user
use syscall::*;

fn write(fd: usize, buffer: &[u8]) -> isize {
    sys_write(fd, buffer)
}

fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
