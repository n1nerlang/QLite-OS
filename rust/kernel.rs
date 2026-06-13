#![no_std]
#![no_main]

mod apps;

use core::fmt::{self, Write};
use core::panic::PanicInfo;

extern "C" {
    fn qlite_runtime_banner() -> *const u8;
    fn qlite_memset(dest: *mut u8, value: i32, count: usize) -> *mut u8;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut writer = VgaWriter::new();
    writer.clear();

    let banner = unsafe { c_str(qlite_runtime_banner()) };
    let _ = writeln!(writer, "{}", banner);
    let _ = writeln!(writer, "Rust + C + asm freestanding prototype");
    let _ = writeln!(writer, "");
    let _ = apps::render_showcase(&mut writer);

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

struct VgaWriter {
    column: usize,
    row: usize,
    color: u8,
    buffer: *mut u16,
}

impl VgaWriter {
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;

    fn new() -> Self {
        Self {
            column: 0,
            row: 0,
            color: 0x0f,
            buffer: 0xb8000 as *mut u16,
        }
    }

    fn clear(&mut self) {
        unsafe {
            qlite_memset(self.buffer as *mut u8, 0, Self::WIDTH * Self::HEIGHT * 2);
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            value => {
                if self.column >= Self::WIDTH {
                    self.new_line();
                }
                let index = self.row * Self::WIDTH + self.column;
                let color = (self.color as u16) << 8;
                unsafe {
                    self.buffer.add(index).write_volatile(color | value as u16);
                }
                self.column += 1;
            }
        }
    }

    fn new_line(&mut self) {
        self.column = 0;
        if self.row + 1 < Self::HEIGHT {
            self.row += 1;
        }
    }
}

impl Write for VgaWriter {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for byte in text.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

unsafe fn c_str(ptr: *const u8) -> &'static str {
    let mut len = 0usize;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len))
}
