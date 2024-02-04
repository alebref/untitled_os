use crate::kernel::console::char_buffer::{CharBuffer, CharColors, PrintableChar};
use crate::kernel::native_graphics::{FrameBuffer, Pixel};
use core::fmt::Write;
use core::panic::PanicInfo;

pub mod char_bitmaps;
mod char_buffer;

impl CharColors {
    const OUTPUT: Self = Self {
        foreground: Pixel::rgb(223, 223, 223),
        background: Pixel::rgb(32, 32, 32),
    };
    const PANIC: Self = Self {
        foreground: Pixel::rgb(223, 223, 223),
        background: Pixel::rgb(223, 0, 0),
    };
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Console {
    char_buffer: CharBuffer,
}

impl Console {
    pub(crate) fn new(frame_buffer: FrameBuffer) -> Self {
        let mut char_buffer = CharBuffer::new(frame_buffer);
        char_buffer.set_char_colors(CharColors::OUTPUT);
        Self { char_buffer }
    }

    pub(crate) fn enter_panic_mode(&mut self) {
        self.char_buffer.set_char_colors(CharColors::PANIC);
    }

    pub(crate) fn print(&mut self, s: &str) {
        for c in s.chars() {
            match c {
                '\n' => self.wrap_line(),
                '\r' => self.go_to_line_start(),
                '\t' => self.insert_tab(),
                _ => self.print_char(c),
            };
        }
    }
    fn wrap_line(&mut self) {
        self.char_buffer.go_down();
        self.char_buffer.go_to_line_start();
    }
    fn go_to_line_start(&mut self) {
        self.char_buffer.go_to_line_start();
    }
    fn insert_tab(&mut self) {
        const TAB_SIZE: usize = 4;
        for _ in 0..TAB_SIZE {
            self.char_buffer.put_char(PrintableChar::SPACE);
        }
    }
    fn print_char(&mut self, c: char) {
        const MAX_UTF8_BYTE_COUNT: usize = 4;
        let mut buffer = [0u8; MAX_UTF8_BYTE_COUNT];
        let utf8_str = c.encode_utf8(&mut buffer);
        for byte in utf8_str.bytes() {
            let printable_char = PrintableChar::try_from(byte);
            if let Ok(printable_char) = printable_char {
                self.char_buffer.put_char(printable_char);
            }
        }
    }
}

struct PanicWriter(Console);

impl Write for PanicWriter {
    /// Never fails
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut console = self.0;
        console.enter_panic_mode();
        console.print(s);
        Ok(())
    }
}

pub(crate) struct DisposablePanicWriter(PanicWriter);

impl DisposablePanicWriter {
    pub(crate) fn new(console: Console) -> Self {
        Self(PanicWriter(console))
    }
    pub(crate) fn panic(mut self, panic_info: &PanicInfo) {
        write!(self.0, "\n{:?}", panic_info).unwrap();
    }
}
