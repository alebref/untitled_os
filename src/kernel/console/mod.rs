use crate::kernel::console::char_bitmaps::{CharBit, CharBitMap, CHAR_RESOLUTION};
use crate::kernel::native_graphics::{FrameBuffer, Pixel, Position};

pub mod char_bitmaps;

#[derive(Clone, Copy, Debug)]
struct CharColor {
    foreground: Pixel,
    background: Pixel,
}

impl CharColor {
    const PANIC: Self = Self {
        foreground: Pixel::RED,
        background: Pixel::WHITE,
    };
}

impl Default for CharColor {
    fn default() -> Self {
        Self {
            foreground: Pixel::WHITE,
            background: Pixel::BLACK,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Console {
    frame_buffer: FrameBuffer,
    cursor_position: Position,
    char_color: CharColor,
}

impl Console {
    pub(crate) fn new(frame_buffer: FrameBuffer) -> Self {
        Self {
            frame_buffer,
            cursor_position: Position {
                horizontal: 0,
                vertical: 0,
            },
            char_color: CharColor::default(),
        }
    }
    pub(crate) fn print(&mut self, s: &str) {
        for c in s.chars() {
            match c {
                '\n' => self.new_line(),
                _ => self.print_char(c),
            };
        }
    }
    pub(crate) fn println(&mut self, s: &str) {
        self.print(s);
        self.new_line();
    }
    pub(crate) fn eprintln(&mut self, s: &str) {
        let current_color = self.char_color;
        self.char_color = CharColor::PANIC;
        self.println(s);
        self.char_color = current_color;
    }
    pub(crate) fn new_line(&mut self) {
        self.cursor_position.horizontal = 0;
        self.cursor_position.vertical += CHAR_RESOLUTION.vertical;
    }
    fn print_char(&mut self, c: char) {
        let mut pixel_position = self.cursor_position;
        for bit_line in CharBitMap::from(c).get_lines() {
            for bit in bit_line.get_bits() {
                let pixel = self.apply_colors(bit);
                self.frame_buffer
                    .draw_pixel_if_visible(pixel_position, pixel);
                pixel_position.horizontal += 1;
            }
            pixel_position.horizontal = self.cursor_position.horizontal;
            pixel_position.vertical += 1;
        }
        self.cursor_position.horizontal += CHAR_RESOLUTION.horizontal;
    }
    fn apply_colors(&self, bit: CharBit) -> Pixel {
        match bit {
            CharBit::Foreground => self.char_color.foreground,
            CharBit::Background => self.char_color.background,
        }
    }
}
