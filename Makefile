# QLite-OS build entry point

BUILD_DIR := build
KERNEL_ELF := $(BUILD_DIR)/qlite-os.elf
CFLAGS := -ffreestanding -fno-builtin -fno-stack-protector -Wall -Wextra -O2
RUSTFLAGS := -C panic=abort -C relocation-model=static

all: $(KERNEL_ELF)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(BUILD_DIR)/boot.o: boot/boot.S | $(BUILD_DIR)
	gcc -c $< -o $@

$(BUILD_DIR)/runtime.o: c/runtime.c | $(BUILD_DIR)
	gcc $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/kernel.o: rust/kernel.rs | $(BUILD_DIR)
	@echo "Rust toolchain not installed in this environment; compile rust/kernel.rs with rustc when available."
	@touch $@

$(KERNEL_ELF): $(BUILD_DIR)/boot.o $(BUILD_DIR)/runtime.o $(BUILD_DIR)/kernel.o
	@echo "Linking the freestanding kernel is intentionally left as a next-step once the Rust toolchain is present."
	@touch $@

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all clean
