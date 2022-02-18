//! # user shell: osh
//!
//! orca's default shell program, providing capability of running different command,
//! doing some tiny jobs as a script language.
//!

#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::osh::Command;
use user_lib::{exec, fork, osh, waitpid};

#[no_mangle]
fn main() -> i32 {
    loop {
        print!("$ ");
        let command = osh::get_command();
        if let Some(command) = command {
            match command {
                Command::Bin(bin) => {
                    let pid = fork();
                    if pid == 0 {
                        // child process
                        if exec(bin.as_str()) == -1 {
                            println!("Error when executing...");
                            return -4;
                        } else {
                            // father process
                            let mut exit_code = 0;
                            let exit_pid = waitpid(pid as usize, &mut exit_code);
                            assert_eq!(exit_pid, pid);
                            println!("Shell: Process {} exit with code {}", pid, exit_code);
                        }
                    }
                }
            }
        } else {
            println!("Unknown command");
        }
    }
}
