#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
pub mod console;
mod heap_allocator;
mod lang_item;
pub mod osh;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    heap_allocator::init();
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

use alloc::string::String;
/// syscall for user
use syscall::*;

pub fn read(fd: usize, buffer: &mut [u8]) -> isize {
    sys_read(fd, buffer)
}

pub fn write(fd: usize, buffer: &[u8]) -> isize {
    sys_write(fd, buffer)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

/// `yield` is a keyword of rust, we have to use another name for this function
pub fn user_yield() -> isize {
    sys_yield()
}

pub fn time() -> isize {
    sys_time()
}

pub fn getpid() -> isize {
    sys_getpid()
}

pub fn fork() -> isize {
    sys_fork()
}

pub fn exec(path: &str) -> isize {
    sys_exec(path)
}

pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => user_yield(),
            exit_pid => return exit_pid,
        };
    }
}

pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            -2 => user_yield(),
            exit_pid => return exit_pid,
        };
    }
}

pub fn sleep(time_ms: usize) {
    let start = sys_time();
    while time() < start + time_ms as isize {
        sys_yield();
    }
}

pub fn readline() -> String {
    console::getline()
}
