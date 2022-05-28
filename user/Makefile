TARGET := riscv64gc-unknown-none-elf
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/release
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

OBJCOPY := rust-objcopy --binary-architecture=riscv64

build: $(APPS)
	@echo "[1/3] Build"
	@cargo build --release
	@echo "[2/3] Changging file"
	# $(foreach elf, $(ELFS), $(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf));)
	@echo "[3/3] Finished"

clean:
	cargo clean

.PHONY: build clean
