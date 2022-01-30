# Make Commands
# 	- make build: compile os
# 	- make qemu: run os and stop when qemu start
# 	- make gdb: open gdb and connect to os which is started
#


QEMU = qemu-system-riscv64
QEMUOPTS = -machine virt \
		   -nographic \
		   -bios bootloader/rustsbi-qemu.bin \
		   -device loader,file=target/riscv64gc-unknown-none-elf/release/orca.bin,addr=0x80200000
# QEMUOPTS += -s -S

GDB = riscv64-unknown-elf-gdb
GDBOPTS = -ex 'file target/riscv64gc-unknown-none-elf/release/orca'
GDBOPTS += -ex 'set arch riscv:rv64'
GDBOPTS += -ex 'target remote localhost:1234'

build:
	@echo "[1/2] Building orca..."
	@cargo build --release
	@echo "[2/2] Modifying os image..."
	@rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/orca -O binary target/riscv64gc-unknown-none-elf/release/orca.bin

qemu:
	@echo "Booting system..."
	$(QEMU) $(QEMUOPTS)

gdb:
	$(GDB) $(GDBOPTS)


all: build

