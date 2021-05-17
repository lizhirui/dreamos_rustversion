target := riscv64imac-unknown-none-elf
mode := debug
root := ../../../
bsp := qemu_virt_rv64
kernel := target/$(target)/$(mode)/$(bsp)
kernel_elf := target/$(target)/$(mode)/dreamos.elf
bin := target/$(target)/$(mode)/dreamos.bin
dump := target/$(target)/$(mode)/dump.asm

objdump := rust-objdump --arch-name=riscv64
objcopy := rust-objcopy --binary-architecture=riscv64

.PHONY: env kernel dump qemu run qemu-dbg dbg gdb

default: qemu

env:
	cargo install cargo-binutils
	rustup component add llvm-tools-preview rustfmt
	rustup target add $(target)

kernel:
	cargo build

$(kernel_elf) : kernel
	\cp -rf $(kernel) $(kernel_elf)

$(bin): $(kernel_elf)
	$(objcopy) $(kernel_elf) --strip-all -O binary $@

dump:
	$(objdump) -d $(kernel_elf) > $(dump)

build: $(bin)

clean:
	cargo clean

qemu: $(bin)
	qemu-system-riscv64 -nographic -machine virt -m 256M -kernel $(bin)

qemu-dbg: $(bin)
	qemu-system-riscv64 -s -S -nographic -machine virt -m 256M -kernel $(bin)

run: qemu

dbg: qemu-dbg

gdb:
	gdbgui -p 10002 -r $(kernel_elf) -g "riscv64-unknown-elf-gdb --command=gdbcommand.txt"
