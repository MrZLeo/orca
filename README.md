# Orca 虎鲸
A RISC-V and unix-like operating system developed just for fun.

This OS have some features:
- Rust
- RISC-V ISA
- Based on rcore

## Schedule
**In the beginning, orca will follow rcore tutorial to implement basic functions. There are few stages you can find in git-log that according to chapters in rcore tutorial.**
1. Basic os that can print messages by uart(you can use differnet color to output messages in kernel)
    ```bash
    git checkout c1ba7a0b2f0829ebe878a0eff856f1a51b21b901
    ```
2. Batch os that can run different user applications one by one(limited syscall supported)
    ```bash
    git checkout 31b4a9b396336b94defb9404091122933b5bcd75
    ```
3. OS that allows time slice and change differnet task to run
    ```bash
    git checkout 94118420a1f709e71bd7f2c355dd73f1601ab63b
    ```
4. Virtual memory support (developing...)

## References
1. xv6-riscv: an elegant educational os https://github.com/mit-pdos/xv6-riscv
2. rcore: an educational os developed by rust https://github.com/rcore-os/rCore
3. rcore-tutorial-v3: https://github.com/rcore-os/rCore-Tutorial-v3/tree/ch2-dev/os/src
