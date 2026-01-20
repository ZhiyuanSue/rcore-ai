//! RISC-V SBI (Supervisor Binary Interface) 调用封装

#![allow(unused)]  // 允许未使用的代码（在最小化内核中）

/// SBI调用类型
#[derive(Debug)]
#[repr(usize)]
pub enum SbiCall {
    SetTimer = 0,
    ConsolePutchar = 1,
    ConsoleGetchar = 2,
    ClearIpi = 3,
    SendIpi = 4,
    RemoteFenceI = 5,
    RemoteSfenceVma = 6,
    RemoteSfenceVmaAsid = 7,
    SystemReset = 8,
}

/// SBI错误码
#[derive(Debug, Clone, Copy)]
#[repr(isize)]
pub enum SbiError {
    Failed = -1,
    NotSupported = -2,
    InvalidParam = -3,
    Denied = -4,
    InvalidAddress = -5,
    AlreadyAvailable = -6,
}

/// 执行SBI调用（内联汇编）
#[inline(always)]
fn sbi_call(which: SbiCall, arg0: usize, arg1: usize, arg2: usize) -> (isize, isize) {
    let mut error: isize;
    let mut value: isize;
    
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") arg0 as isize => error,
            inlateout("x11") arg1 as isize => value,
            in("x12") arg2,
            in("x17") which as usize,
        );
    }
    
    (error, value)
}

/// 向控制台输出一个字符
pub fn console_putchar(c: u8) {
    sbi_call(SbiCall::ConsolePutchar, c as usize, 0, 0);
}

/// 关闭计算机（关机）
pub fn shutdown(what: usize) -> Result<(), SbiError> {
    let (error, _) = sbi_call(SbiCall::SystemReset, what, 0, 0);
    
    match error {
        0 => Ok(()),
        -1 => Err(SbiError::Failed),
        -2 => Err(SbiError::NotSupported),
        -3 => Err(SbiError::InvalidParam),
        -4 => Err(SbiError::Denied),
        _ => Err(SbiError::Failed),
    }
}

/// 简单的关机函数（提供给主函数使用）
pub fn shutdown_simple() -> ! {
    // 关机类型：0=关机，1=重启，2=关机
    let _ = shutdown(0);
    
    // 如果关机失败，进入死循环
    loop {
        unsafe { core::arch::asm!("wfi"); } // 等待中断（省电）
    }
}
