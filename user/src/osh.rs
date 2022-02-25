//! # Osh implementation
//!
//! osh is a general command shell for orca, it will follow architecture:
//! `OshLexer` --> `OshParser` --> `shell.rs`(binary program in `bin/`)

#![macro_use]
extern crate alloc;
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
pub fn get_command() -> Option<Command> {
    None
}

enum Token {
    Num(i32),
    Literal(String),
}

struct PeekInterator<T> {
    iter: Arc<dyn Iterator<Item = T>>,
    back: LinkedList<T>,
}

struct OshLexer {
    cur_token: Option<Token>,
    iter: PeekInterator<char>,
}

impl OshLexer {}

struct OshParser {}
