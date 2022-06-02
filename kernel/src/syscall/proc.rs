use alloc::format;
use alloc::sync::Arc;

use crate::console::{println_with_color, YELLOW};
use crate::fs::inode::{open_file, OpenFlags};
use crate::mm::page_table;
use crate::mm::page_table::translated_str;
use crate::task::exit_cur_and_run_next;
use crate::task::processor::{cur_task, cur_user_token};
use crate::task::{self, processor, scheduler, task::ProcessControlBlock};
use crate::timer::time_ms;

pub fn sys_exit(exit_code: i32) -> ! {
    exit_cur_and_run_next(exit_code);
    panic!("Unreachable: app exited");
}

pub fn sys_yield() -> isize {
    task::suspend_cur_and_run_next();
    0
}

pub fn sys_time() -> isize {
    time_ms() as isize
}

pub fn sys_fork() -> isize {
    let cur_task = processor::cur_task().unwrap();
    let child = ProcessControlBlock::fork(&cur_task);
    let pid = child.pid.0;

    // set child return code = 0
    let trap_cxt = child.borrow_mut().trap_cxt();
    trap_cxt.x[10] = 0;
    scheduler::add_task(child);

    pid as isize
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = cur_task().unwrap();

    let mut inner = task.borrow_mut();
    let child = inner
        .children
        .iter()
        .enumerate()
        .find(|&(_, p)| pid == -1 || p.getpid() == pid as usize);
    if let Some((idx, p)) = child {
        if p.borrow_mut().is_zombie() {
            let del = inner.children.remove(idx);
            assert_eq!(Arc::strong_count(&del), 1);
            let del_pid = del.getpid();
            let exit_code = del.borrow_mut().exit_code;
            *page_table::translated_refmut(inner.user_token(), exit_code_ptr) = exit_code;
            del_pid as isize
        } else {
            -2
        }
    } else {
        -1
    }
}

pub fn sys_getpid() -> isize {
    let a = cur_task().unwrap();
    a.pid.0 as isize
}

pub fn sys_spawn(path: *const u8) -> isize {
    // TODO
    0
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = cur_user_token();
    let path = translated_str(token, path);
    // read elf file as read only becase we don't want
    // our executable file get modify
    if let Some(app_inode) = open_file(path.as_str(), OpenFlags::RDONLY) {
        let all_data = app_inode.read_all();
        let task = cur_task().unwrap();
        task.exec(all_data.as_slice());
        0
    } else {
        -1
    }
}
