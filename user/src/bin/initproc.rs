#![no_std]
#![no_main]

use user_lib::{exec, fork, user_yield, wait};

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    if fork() == 0 {
        exec("shell\0");
    } else {
        loop {
            let mut exit_code = 0i32;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                user_yield();
                continue;
            }
            println!(
                "[initproc] Released a zombie process pid = {}, exit code = {}",
                pid, exit_code
            );
        }
    }

    0
}
