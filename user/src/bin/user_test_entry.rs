#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

static TESTS: &[&str] = &[
    "exit\0",
    "fantastic_text\0",
    "forktest\0",
    "forktest2\0",
    "forktest_simple\0",
    "forktree\0",
    "hello_world\0",
    "matrix\0",
    "sleep\0",
    "sleep_simple\0",
    "stack_overflow\0",
    "yield\0",
    "filetest_simple\0",
    "cat_filea\0",
    "huge_write\0",
];

use alloc::string::ToString;
use user_lib::{
    console::{print_with_color, println_with_color, BLUE, GREEN, RED},
    exec, fork, waitpid,
};

#[no_mangle]
pub fn main() -> i32 {
    let mut passed: i32 = 0;
    let mut fail: i32 = 0;
    for (i, test) in TESTS.iter().enumerate() {
        println!("[{}] User test: Running {}", i, test);
        let pid = fork();
        if pid == 0 {
            exec(*test);
            panic!("unreachable!");
        } else {
            let mut exit_code: i32 = Default::default();
            let wait_pid = waitpid(pid as usize, &mut exit_code);
            assert_eq!(pid, wait_pid);
            println!(
                "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
                test, pid, exit_code
            );
            if exit_code == 0 {
                passed += 1;
            } else {
                fail += 1;
            }
        }
    }
    println_with_color("[TEST] --- TEST END ---", BLUE);
    print_with_color("[TEST] Test Result: ", BLUE);
    print_with_color("success: ", GREEN);
    let passed = passed.to_string();
    print_with_color(passed.as_str(), GREEN);
    let fail = fail.to_string();
    print!(", ");
    print_with_color("fail: ", RED);
    println_with_color(fail.as_str(), RED);
    0
}
