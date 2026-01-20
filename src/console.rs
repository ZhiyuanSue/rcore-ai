//! 控制台输出模块
//! 实现基于SBI的字符输出功能

use crate::sbi::console_putchar;
use core::fmt;

/// 实现Write trait，使我们可以使用format!宏
pub struct Console;

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            // 简单处理字符：ASCII直接输出，非ASCII输出问号
            if c.is_ascii() {
                console_putchar(c as u8);
            } else {
                console_putchar(b'?');
            }
        }
        Ok(())
    }
}

/// 打印字符串（不换行）
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut console = $crate::console::Console;
        let _ = write!(&mut console, $($arg)*);
    }};
}

/// 打印字符串（换行）
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!($($arg)*);
        $crate::print!("\n");
    }};
}

/// 初始化控制台（目前什么都不做，为后续扩展预留）
pub fn init() {
    // 未来可以在这里初始化串口等硬件
}
