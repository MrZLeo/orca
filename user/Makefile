TARGET := riscv64gc-unknown-none-elf
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/release
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

OBJCOPY := rust-objcopy --binary-architecture=riscv64

build: $(APPS)
	@echo "[1/2] Build"
	@cargo build --release
	@echo "[2/2] Finished"

clean:
	cargo clean

test: $(APPS)
	@echo "test build"
	@cargo build --features "user_test" --release

.PHONY: build clean
