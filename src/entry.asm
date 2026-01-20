/* 告诉链接器我们的入口点是 _start */
ENTRY(_start)

/* 定义内存布局 */
SECTIONS
{
    /* 内核起始地址，这是RISC-V下QEMU加载内核的标准地址 */
    . = 0x80200000;
    
    /* 代码段 */
    .text : {
        *(.text.entry)      /* 入口代码放在最前面 */
        *(.text .text.*)    /* 其他代码 */
    }
    
    /* 只读数据段 */
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    
    /* 数据段 */
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    
    /* 清零的BSS段（未初始化数据） */
    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    
    /* 内核栈空间 */
    . = ALIGN(4K);
    boot_stack = .;
    . += 4K;  /* 4KB栈空间 */
    boot_stack_top = .;
    
    /* 结束符号 */
    . = ALIGN(4K);
    _end = .;
}
