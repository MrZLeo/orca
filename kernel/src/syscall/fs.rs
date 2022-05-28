/// stdin: 0
/// stdout: 1
/// stderr: 2
const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;
const FD_STDERR: usize = 2;

use core::borrow::BorrowMut;

use crate::fs::inode::{self, OpenFlags};
use crate::mm::page_table::{translated_byte_buffer, translated_str, UserBuf};
use crate::sbi::consolo_getchar;
use crate::task::processor::{self, cur_task, cur_user_token};
use crate::task::suspend_cur_and_run_next;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = cur_user_token();
    let task = cur_task().unwrap();
    let inner = task.inner.borrow_mut();

    if fd >= inner.fd_table.len() {
        return -1;
    }

    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();

        drop(inner);

        file.write(UserBuf::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

/// @return the len that read from `fd`
pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = cur_user_token();
    let task = cur_task().unwrap();
    let inner = task.inner.borrow_mut();

    if fd >= inner.fd_table.len() {
        return -1;
    }

    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        drop(inner);
        file.read(UserBuf::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    let task = cur_task().unwrap();
    let token = cur_user_token();
    let path = translated_str(token, path);
    if let Some(inode) = inode::open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
        let mut inner = task.inner.borrow_mut();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let task = cur_task().unwrap();
    let mut inner = task.inner.borrow_mut();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}
