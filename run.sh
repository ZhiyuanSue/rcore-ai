#!/bin/bash
# 运行QEMU脚本

KERNEL=target/riscv64gc-unknown-none-elf/debug/kernel

if [ "$1" = "release" ]; then
    KERNEL=target/riscv64gc-unknown-none-elf/release/kernel
fi

# 使用QEMU运行
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios default \
    -kernel $KERNEL \
    -smp 1 \
    -m 128M
