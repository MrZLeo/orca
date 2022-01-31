TARGET := riscv64gc-unknown-none-elf
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET_DIR)/release
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

OBJCOPY := rust-cbjcopy --binary-architecture=riscv64

build:
	@echo "[1/3] Build"
	@cargo build --release
	@echo "[2/3] Changging file"
	@echo $(foreach elf, $(ELFS), $(OBJCOPY) $(elf) --strip-all -O binary $(elf).bin)
	@echo "[3/3] Finished"

