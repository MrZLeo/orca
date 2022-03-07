use alloc::format;
use alloc::sync::Arc;

use crate::console::{println_with_color, YELLOW};
use crate::loader::app_from_name;
use crate::mm::page_table;
use crate::task::exit_cur_and_run_next;
use crate::task::processor::{cur_task, cur_user_token};
use crate::task::{self, processor, scheduler, task::ProcessControlBlock};
use crate::timer::time_ms;

pub fn sys_exit(exit_code: i32) -> ! {
    println_with_color(
        format!("[kernel]Process exited with code {}", exit_code).as_str(),
        YELLOW,
    );
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

pub fn sys_exec(path: *const u8) -> isize {
    let token = cur_user_token();
    let command = page_table::translated_str(token, path);
    if let Some(data) = app_from_name(command.as_str()) {
        let task = cur_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
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
