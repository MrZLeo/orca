use std::fs::{read_dir, File};
use std::io::{Result, Write};

static TARGET_DIR: &str = "../user/target/riscv64gc-unknown-none-elf/release/";

/// crate `link_app.S` automatically
fn init_app_data() -> Result<()> {
    let mut file = File::create("link_app.S").unwrap();

    // get file name `xxx.bin`, and reduce `.bin`
    // all name store in `apps`
    let mut apps: Vec<_> = read_dir("../user/src/bin")
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let mut name = dir_entry.unwrap().file_name().into_string().unwrap();
            name.drain(name.find('.').unwrap()..name.len());
            name
        })
        .collect();

    // make sure that all user apps will be execute by their number
    apps.sort();

    // produce link_app.S by user apps
    writeln!(
        file,
        r#"
    .align 3 # 当前PC地址推进到“2的3次方(8)个字节”对齐的位置
    .section .data
    .global _num_app
_num_app:
    .quad {} # 在存储器中分配8个字节，用apps.len()对存储单元进行初始化"#,
        apps.len()
    )?;

    for i in 0..apps.len() {
        writeln!(file, r#"    .quad app_{}_start"#, i)?;
    }
    writeln!(file, r#"    .quad app_{}_end"#, apps.len() - 1)?;

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            file,
            r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin" # include file `{1}.bin`
app_{0}_end:"#,
            idx, app, TARGET_DIR
        )?;
    }

    Ok(())
}

fn main() {
    init_app_data().unwrap();
}
