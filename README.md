# Orca 🐋 虎鲸

A RISC-V and unix-like operating system developed just for fun.

This OS have some features:
- Rust
- RISC-V ISA
- Based on rcore

## Get Start

In order to set enviroment correctly, you should pay attention to your toolchains carefully.

### Rust toolchains

- rustc nightly(I will keep following the newest rustc version, until 2022-2-14, it is vers 1.60)
- risc-v target: we will install them automatically by make commands which will be introduced next
- other utilities: we will install them automatically by make commands which will introduced next

### Make Commands

There are lots of make commands(which will grow together with kernel), and I list them here to make you understand what to do.

#### Kernel Make Commands

- `make build`: compile os
- `make qemu`: run os
- `make debug`: build in debug mode
- `make gdb`: open gdb and connect to os which is started, **os must be build in debug mode**
- `make test`: build and run test
- `make run`: build and qemu
- `make env`: build the basic environment for rust compiler
- `make clean`: remove target directory

#### User Make Commands

- `make build`: compile user codes and modify the binary if necessary
- `make clean`: remove target directory

## Change Log

**In the beginning, orca will follow rcore tutorial to implement basic functions. There are few stages you can find in git-log that according to chapters in rcore tutorial.**

### 2022-1-31
Basic os that can print messages by uart(you can use differnet color to output messages in kernel)

```
git checkout c1ba7a0b2f0829ebe878a0eff856f1a51b21b901
```

### 2022-2-3
Batch os that can run different user applications one by one(limited syscall supported)

```
git checkout 31b4a9b396336b94defb9404091122933b5bcd75
```

### 2022-2-7
OS that allows time slice and change differnet task to run

```
git checkout 94118420a1f709e71bd7f2c355dd73f1601ab63b
```

### 2022-2-8 Test Architecture for orca
This is a test architecture for orca, which is simple but good enough to support orca kernel test.

In test directory, you can design your own test module and bind it to `mod.rs`. It is better to name your module like `xx_test.rs`

In your module, you should desigin a interface like `xx_test`, which contains your whole test procedure. Pay attention that your test function must be use by `test_fn`, which can help us records test result. After that, call it in `main` of `mod.rs`

Test functions is used like `assert!()` macro, but `assert` macro will panic if test don't pass, which can't be use to records test result.

1. Define test module `mm_test.rs` in directory `test`
2. Define test interface `mm_test`
3. Define variable `MM_TEST_NUM` to store the number of test in this module
4. Define two test function: `heap_test` and `heap_test2`
5. In `mm_test()`, call test function:
```rust
pub fn mm_test() {
    test!("Memory Test Start: Running {} test\n", MM_TEST_NUM);
    test!("heap test1...");
    test_fn(heap_test);
    test!("heap test2...");
    test_fn(heap_test2);
}
```
6. import my module in `mod.rs`, call the interface `mm_test`

```
git checkout a4eceef4b0f8f95f8b231bf47a6298e55813939e
```


### 2022-2-12 Virtual memory support
Orca release new version that support virtual address space. It is still a initial version, and there are few works to do in future.

Until now, these works is still transparent outside the OS.

- more capable syscall: new systime, mmap, munmap...
- more efficent task scheduler
- more efficent frame allocator
- **posix interface**(next version!)

```
git checkout 504f206d1b891182c8de5599506df2e10e5f8232
```

### 2022-3-7 Process Management and Shell Support
Orca release new version to support process and typical shell interaction mode. Now we can program useful command
and compile it with kernel, and shell will provide it as built-in binary.

```
git checkout a183e1e39552d056dd78b81724b4ebf496235896
```

![shell-support](https://raw.githubusercontent.com/MrZLeo/Image/main/uPic/2022/03/07/shell-supportkpzugR.gif)

### 2022-3 Way to Persistent Storage
We will develop a file system that allow us to store of os.
This file system should be easy to develop and powerful enought right now.
And it will use extern crate like `lru` and `spin` to achieve efficiency.


## References

1. xv6-riscv: an elegant educational os https://github.com/mit-pdos/xv6-riscv
2. rcore: an educational os developed by rust https://github.com/rcore-os/rCore
3. rcore-tutorial-v3: https://github.com/rcore-os/rCore-Tutorial-v3/tree/ch2-dev/os/src
