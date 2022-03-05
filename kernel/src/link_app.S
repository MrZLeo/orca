
    .align 3
# 当前PC地址推进到“2的3次方(8)个字节”对齐的位置
    .section .data
    .global _num_app
_num_app:
    .quad 2
# 在存储器中分配8个字节，用apps.len()对存储单元进行初始化
    .quad app_0_start
    .quad app_1_start
    .quad app_1_end

    .global _app_names
_app_names:
    .string "initproc"
    .string "shell"

    .section .data
    .global app_0_start
    .global app_0_end
    .align 3
app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/initproc"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
    .align 3
app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/shell"
app_1_end:
