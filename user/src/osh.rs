//! # Osh implementation
//!
//! osh is a general command shell for orca, it will follow architecture:
//! `OshLexer` --> `OshParser` --> `shell.rs`(binary program in `bin/`)

#![macro_use]
extern crate alloc;

use crate::console::getline;
use alloc::string::String;

// FIXME: should be generate automatically
static BUILTIN_BIN: &[&str] = &[
    "exit\0",
    "fantastic_text\0",
    "forktest\0",
    "forktest2\0",
    "forktest_simple\0",
    "hello_world\0",
    "matrix\0",
    "sleep\0",
    "sleep_simple\0",
    "stack_overflow\0",
    "yield\0",
    "usertests\0",
];

// use crate::console::BS;
// use crate::console::CR;
// use crate::console::DL;
// use crate::console::LF;

// TODO
pub enum Command {
    Bin(String),
}

/// Public Interface of osh
/// - Produce a `Command` by analyzing a code block
/// - A code block can be a line of code or more complicated structure like
/// *`For` loop* or *`if` condition*
/// TODO: now it is a temporary implementation, just read command
pub fn get_command() -> Option<Command> {
    let str = getline();
    if BUILTIN_BIN.iter().any(|&bin| *bin == str) {
        Some(Command::Bin(str))
    } else {
        print!("{str}: ");
        None
    }
}

enum Token {
    Num(i32),
    Literal(String),
}

struct OshLexer {
    cur_token: Option<Token>,
    // iter: PeekInterator<char>,
}

impl OshLexer {}

struct OshParser {}
