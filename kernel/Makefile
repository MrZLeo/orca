# Make Commands
# 	- make build: compile os
# 	- make qemu: run os and stop when qemu start
# 	- make gdb: open gdb and connect to os which is started
#   - make eun: build and qemu
#   - env: build the basic environment for rust compiler

TARGET := riscv64gc-unknown-none-elf

QEMU = qemu-system-riscv64
QEMUOPTS = -machine virt \
		   -nographic \
		   -bios ../bootloader/rustsbi-qemu.bin \
		   -device loader,file=target/$(TARGET)/release/orca.bin,addr=0x80200000
# QEMUOPTS += -s -S

GDB = riscv64-unknown-elf-gdb
GDBOPTS = -ex 'file target/riscv64gc-unknown-none-elf/release/orca'
GDBOPTS += -ex 'set arch riscv:rv64'
GDBOPTS += -ex 'target remote localhost:1234'

build:
	@echo "[1/3] building user apps..."
	@cd ../user && make build
	@echo "[2/3] Building orca..."
	@cargo build --release
	@echo "[3/3] Modifying os image..."
	@rust-objcopy --strip-all target/$(TARGET)/release/orca -O binary target/riscv64gc-unknown-none-elf/release/orca.bin

qemu:
	@echo "[1/1] Booting system..."
	@$(QEMU) $(QEMUOPTS)

run: build qemu

gdb:
	$(GDB) $(GDBOPTS)

env:
	@echo "\x1b[1m[1/4] Check target\x1b[0m"
	@(rustup target list | grep "$(TARGET) (installed)") || @rustup target add $(TARGET)
	@echo "\x1b[1m[2/4] Check cargo-binutils\x1b[0m"
	@cargo install cargo-binutils
	@echo "\x1b[1m[3/4] Check rust-src\x1b[0m"
	@rustup component add rust-src
	@echo "\x1b[1m[4/4] Check llvm-tools-preview\x1b[0m"
	@rustup component add llvm-tools-preview


all: build

