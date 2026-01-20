//! RISC-V Hello World 内核
//! 这个内核会在QEMU中启动并打印"Hello World!"

#![no_std]      // 不使用标准库
#![no_main]     // 不使用标准main函数
#![feature(panic_info_message)]  // 启用panic信息特性

// 导入模块
mod sbi;
mod console;

// 导入汇编入口点
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

// 导入println宏
use console::println;

/// 内核主函数
/// 注意：这个函数永远不会返回（-> ! 表示发散函数）
#[no_mangle]  // 防止Rust编译器重命名这个函数
pub extern "C" fn rust_main() -> ! {
    // 打印Hello World!
    println!("========================================");
    println!("Hello, rCore World from RISC-V!");
    println!("This is a minimal OS kernel.");
    println!("========================================");
    
    // 打印一些调试信息
    println!("");
    println!("Kernel is running in bare-metal mode.");
    println!("No standard library available.");
    println!("All output goes through SBI calls.");
    
    // 关机
    println!("");
    println!("Shutting down...");
    sbi::shutdown_simple();
    
    // 永远不会执行到这里
    loop {}
}

/// Panic处理函数
/// 当程序发生无法处理的错误时调用
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("\n!!! KERNEL PANIC !!!");
    
    // 打印panic位置
    if let Some(location) = info.location() {
        println!("Location: {}:{}:{}", 
            location.file(), 
            location.line(), 
            location.column()
        );
    }
    
    // 打印panic信息
    if let Some(message) = info.message() {
        println!("Reason: {}", message);
    }
    
    println!("System halted.");
    
    // 关机
    sbi::shutdown_simple();
}
