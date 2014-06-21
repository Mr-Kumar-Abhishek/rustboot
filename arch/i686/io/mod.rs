use core::option::{Some, None};
use core::str::StrSlice;
use core::slice::ImmutableVector;
use core::iter::Iterator;
use core::fmt;
use core::result::Ok;

use super::drivers::vga;

static mut pos: int = 0;

unsafe fn seek(offset: int) {
    pos += offset;
    vga::cursor_at(pos as uint);
}

unsafe fn write_char(c: char) {
    if c == '\x08' {
        if pos > 0 {
            if pos % 80 == 0 {
                while (*vga::SCREEN)[pos as uint].char == 0 {
                    pos -= 1;
                }
            }
            else if pos > 0 {
                pos -= 1;
                (*vga::SCREEN)[pos as uint].char = 0;
            }
        }
    }
    else if c == '\n' {
        seek(80 - pos % 80);
    }
    else if c == '\t' {
        seek(4 - pos % 4);
    }
    else {
        (*vga::SCREEN)[pos as uint].char = c as u8;
        pos += 1;
    }

    pos %= vga::SCREEN_SIZE as int;
    vga::cursor_at(pos as uint);
}

pub fn putc(c: u8) {
    unsafe {
        write_char(c as char);
    }
}

struct Writer;

impl Writer {
    fn write_fmt(&mut self, fmt: &fmt::Arguments) {
        fmt::write(self, fmt);
    }
}

impl fmt::FormatWriter for Writer {
    fn write(&mut self, bytes: &[u8]) -> fmt::Result {
        for &c in bytes.iter() {
            putc(c);
        }
        Ok(())
    }
}

pub fn println_args(fmt: &fmt::Arguments) {
    writeln!(Writer, "{}", fmt);
}