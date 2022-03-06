//! # Osh implementation
//!
//! osh is a general command shell for orca, it will follow architecture:
//! `OshLexer` --> `OshParser` --> `shell.rs`(binary program in `bin/`)

#![macro_use]
extern crate alloc;

use crate::console::getline;
use alloc::{collections::LinkedList, string::String, sync::Arc};

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
    Some(Command::Bin(str))
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
