use crate::{
    fs::inode::{open_file, OpenFlags},
    mm::page_table::translated_str,
    task::processor::{cur_task, cur_user_token},
};

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
