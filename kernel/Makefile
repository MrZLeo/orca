# Make Commands
# 	- make build: compile os
# 	- make qemu: run os and stop when qemu start
# 	- make debug: build in debug mode
# 	- make gdb: open gdb and connect to os which is started, **os must be build in debug mode**
# 	- make qeun: build and qemu
# 	- make env: build the basic environment for rust compiler

TARGET := riscv64gc-unknown-none-elf
OS_NAME := orca
OS_BIN := $(OS_NAME).bin
RELEASE_DIR := target/$(TARGET)/release
DEBUG_DIR := $(subst release,debug,$(RELEASE_DIR))

BOOTLOADER_DIR := ../bootloader
BOOTLOADER_BIN := rustsbi-qemu.bin

BASE_ADDR := 0x80200000

QEMU = qemu-system-riscv64
QEMUOPTS = -machine virt \
		   -nographic \
		   -bios $(BOOTLOADER_DIR)/$(BOOTLOADER_BIN) \
		   -device loader,file=$(RELEASE_DIR)/$(OS_BIN),addr=$(BASE_ADDR)

QEMUOPTS_DEBUG := $(subst release,debug,$(QEMUOPTS))
QEMUOPTS_DEBUG += -s -S

GDB = riscv64-unknown-elf-gdb
GDBOPTS = -ex 'file $(DEBUG_DIR)/$(OS_NAME)'
GDBOPTS += -ex 'set arch riscv:rv64'
GDBOPTS += -ex 'target remote localhost:1234'


build:
	@echo "### building user apps..."
	@cd ../user && make build
	@echo "### Building orca..."
	@cargo build --release
	@echo "### Modifying os image..."
	@rust-objcopy --strip-all $(RELEASE_DIR)/$(OS_NAME) -O binary $(RELEASE_DIR)/$(OS_BIN)

qemu:
	@echo "### Booting system..."
	@$(QEMU) $(QEMUOPTS)

run: build qemu

debug:
	cd ../user && make build
	cargo build
	rust-objcopy --strip-all $(DEBUG_DIR)/$(OS_NAME) -O binary $(DEBUG_DIR)/$(OS_BIN)
	$(QEMU) $(QEMUOPTS_DEBUG)

gdb:
	$(GDB) $(GDBOPTS)

env:
	@echo "\x1b[1m### Check target\x1b[0m"
	@(rustup target list | grep "$(TARGET) (installed)") || @rustup target add $(TARGET)
	@echo "\x1b[1m### Check cargo-binutils\x1b[0m"
	@cargo install cargo-binutils
	@echo "\x1b[1m### Check rust-src\x1b[0m"
	@rustup component add rust-src
	@echo "\x1b[1m### Check llvm-tools-preview\x1b[0m"
	@rustup component add llvm-tools-preview


all: build

