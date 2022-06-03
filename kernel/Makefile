# Make Commands
# 	- make build: compile os
# 	- make qemu: run os and stop when qemu start
# 	- make debug: build in debug mode
# 	- make gdb: open gdb and connect to os which is started, **os must be build in debug mode**
# 	- make test: build and run test
# 	- make qeun: build and qemu
# 	- make env: build the basic environment for rust compiler
# 	- make img: build file system image

TARGET := riscv64gc-unknown-none-elf
OS_NAME := orca
OS_BIN := $(OS_NAME).bin
RELEASE_DIR := target/$(TARGET)/release
DEBUG_DIR := $(subst release,debug,$(RELEASE_DIR))
USER_DIR := ../user
USER_TARGET := $(USER_DIR)/target/riscv64gc-unknown-none-elf/release/
APPS := $(USER_DIR)/src/bin/*

BOOTLOADER_DIR := ../bootloader
BOOTLOADER_BIN := rustsbi-qemu.bin

BASE_ADDR := 0x80200000

FS_IMG := ../user/$(RELEASE_DIR)/fs.img

QEMU = qemu-system-riscv64
QEMUOPTS = -machine virt \
		   -nographic \
		   -m 128M \
		   -bios $(BOOTLOADER_DIR)/$(BOOTLOADER_BIN) \
		   -device loader,file=$(RELEASE_DIR)/$(OS_BIN),addr=$(BASE_ADDR) \

QEMUOPTS_TEMP := $(QEMUOPTS)
QEMUOPTS_DEBUG := $(subst release,debug,$(QEMUOPTS_TEMP))
QEMUOPTS_DEBUG += -s -S

QEMUOPTS += -drive file=$(FS_IMG),if=none,format=raw,id=x0 \
		    -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

QEMUOPTS_DEBUG += -drive file=$(FS_IMG),if=none,format=raw,id=x0 \
		    -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

GDB = riscv64-unknown-elf-gdb
GDBOPTS = -ex 'file $(DEBUG_DIR)/$(OS_NAME)'
GDBOPTS += -ex 'set arch riscv:rv64'
GDBOPTS += -ex 'target remote localhost:1234'

user:
	@echo "### building user apps..."
	@cd ../user && make build

qemu:
	@echo "### Booting system..."
	$(QEMU) $(QEMUOPTS)

release_build:
	@echo "### Building orca..."
	@cargo build --release

release_objcopy:
	@echo "### Modifying os image..."
	@rust-objcopy --strip-all $(RELEASE_DIR)/$(OS_NAME) -O binary $(RELEASE_DIR)/$(OS_BIN)

build: release_build release_objcopy

run: img build qemu

debug_build:
	@echo "### Building orca..."
	cargo build

debug_objcopy:
	@echo "### Modifying os image..."
	rust-objcopy --strip-all $(DEBUG_DIR)/$(OS_NAME) -O binary $(DEBUG_DIR)/$(OS_BIN)

debug_qemu:
	$(QEMU) $(QEMUOPTS_DEBUG)

debug: img debug_build debug_objcopy debug_qemu

test_build:
	@echo "### Building orca..."
	@cargo build --features "kernel_test" --release


test: img_test test_build release_objcopy qemu

gdb:
	$(GDB) $(GDBOPTS)

env:
	@echo "### Check target"
	@(rustup target list | grep "$(TARGET) (installed)") || rustup target add $(TARGET)
	@echo "### Check cargo-binutils"
	@cargo install cargo-binutils
	@echo "### Check rust-src"
	@rustup component add rust-src
	@echo "### Check llvm-tools-preview"
	@rustup component add llvm-tools-preview

kernel_check:
	@echo "### Checking kernel..."
	@cargo clippy

user_check:
	@echo "### Checking user..."
	@cd ../user && cargo clippy

check: user_check kernel_check

clean:
	cd ../user && make clean
	cargo clean

all: build

img: $(APPS)
	@cd ../user && make build
	@rm -rf $(FS_IMG)
	@cd ../easy-fs-test-by-rcore && cargo run --release -- -s $(USER_DIR)/src/bin/ -t ../user/target/riscv64gc-unknown-none-elf/release/

$(APPS):

img_test: $(APPS)
	@cd ../user && make test
	@rm -rf $(FS_IMG)
	@cd ../easy-fs-test-by-rcore && cargo run --release -- -s $(USER_DIR)/src/bin/ -t ../user/target/riscv64gc-unknown-none-elf/release/


.PHONY: build qemu debug env gdb run all img test img_test
