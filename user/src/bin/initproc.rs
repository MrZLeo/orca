#![no_std]
#![no_main]

use user_lib::{exec, exit, fork, shutdown, user_yield, wait};

#[macro_use]
extern crate user_lib;

/// # Initproc
///
/// It works like a simple `launchd`, which provides management and clearup of
/// all process. In details, it will `fork` a child process to boot shell, which
/// is **osh** here. After that, initproc wait process to exit and do some cleanup.
///
/// - Actually this process **will not return forever**, but our `main` function here
/// must be compatible.
#[no_mangle]
fn main() -> i32 {
    if fork() == 0 {
        /* if we are in test, just call `user_test_entry`, we don't need to
         * boot the shell*/
        if cfg!(feature = "user_test") {
            exec("user_test_entry\0");
        } else {
            exec("shell\0");
        }
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
            /* if we are in test just exit after all test finished */
            if cfg!(feature = "user_test") {
                shutdown();
            }
        }
    }

    0
}
