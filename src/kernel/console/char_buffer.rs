use crate::kernel::console::char_bitmaps::{CharBit, CharBitMap, CHAR_RESOLUTION};
use crate::kernel::native_graphics::{FrameBuffer, Pixel, PixelPosition, Resolution};

#[derive(Clone, Copy, Debug)]
pub(super) struct CharColors {
    pub(super) foreground: Pixel,
    pub(super) background: Pixel,
}

impl CharColors {
    const NONE: Self = Self {
        foreground: Pixel::BLACK,
        background: Pixel::BLACK,
    };

    const DEFAULT: Self = Self {
        foreground: Pixel::WHITE,
        background: Pixel::BLACK,
    };

    const fn apply(&self, bit: CharBit) -> Pixel {
        match bit {
            CharBit::Foreground => self.foreground,
            CharBit::Background => self.background,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct CharPosition {
    frame_resolution: Resolution,
    /// Pixel position of the upper right corner of the character
    pixel_position: PixelPosition,
}

impl CharPosition {
    fn initial(frame_resolution: Resolution) -> Self {
        const INITIAL_POSITION: PixelPosition = PixelPosition {
            horizontal: 0,
            vertical: 0,
        };
        Self {
            frame_resolution,
            pixel_position: INITIAL_POSITION,
        }
    }

    fn the_whole_char_can_be_drawn_vertically(&self) -> bool {
        self.pixel_position.vertical + CHAR_RESOLUTION.vertical < self.frame_resolution.vertical
    }

    fn go_to_unchecked(&mut self, row_index: usize, column_index: usize) {
        self.pixel_position = PixelPosition {
            horizontal: column_index * CHAR_RESOLUTION.horizontal,
            vertical: row_index * CHAR_RESOLUTION.vertical,
        }
    }

    fn go_right_or_start_new_line(&mut self) {
        self.pixel_position.horizontal += CHAR_RESOLUTION.horizontal;
        if self.pixel_position.horizontal >= self.frame_resolution.horizontal {
            self.go_down();
            self.go_back_to_start_of_line();
        }
    }
    fn go_down(&mut self) {
        self.pixel_position.vertical += CHAR_RESOLUTION.vertical;
    }
    fn go_back_to_start_of_line(&mut self) {
        self.pixel_position.horizontal = 0;
    }
}

/// A printable char is an ASCII char between U+0020 and U+007E
///
/// The space char is printable, as it has a background color, but it is still not visible
#[derive(Clone, Copy, Debug)]
pub(super) struct PrintableChar(u8);

impl PrintableChar {
    const FIRST_PRINTABLE_CHAR: u8 = 0x20;
    const LAST_PRINTABLE_CHAR: u8 = 0x7E;

    const NOT_PRINTABLE_CHAR: u8 = 0;
    const NONE: Self = Self(Self::NOT_PRINTABLE_CHAR);
    const fn is_none(&self) -> bool {
        self.0 == Self::NOT_PRINTABLE_CHAR
    }

    pub(super) const SPACE: Self = Self(0x20);

    pub(super) const fn get_index_from_first_printable_char(&self) -> usize {
        (self.0 - 0x20) as usize
    }
}

impl TryFrom<u8> for PrintableChar {
    type Error = ();
    /// Succeeds only if `byte` is an ASCII printable char (between U+0020 and U+007E)
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            Self::FIRST_PRINTABLE_CHAR..=Self::LAST_PRINTABLE_CHAR => Ok(PrintableChar(byte)),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct OptionalColoredChar {
    printable_char: PrintableChar,
    colors: CharColors,
}

impl OptionalColoredChar {
    const NONE: Self = Self {
        printable_char: PrintableChar::NONE,
        colors: CharColors::NONE,
    };
    const fn is_none(&self) -> bool {
        self.printable_char.is_none()
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct CharBuffer {
    frame_buffer: FrameBuffer,
    char_colors: CharColors,
    cursor_position: CharPosition,
    width: usize,
    height: usize,
}

impl CharBuffer {
    const MAX_WIDTH: usize = Resolution::MAX_SUPPORTED.horizontal / CHAR_RESOLUTION.horizontal;
    const MAX_HEIGHT: usize = Resolution::MAX_SUPPORTED.vertical / CHAR_RESOLUTION.vertical;

    pub(super) fn new(frame_buffer: FrameBuffer) -> Self {
        let width = frame_buffer.resolution.horizontal / CHAR_RESOLUTION.horizontal;
        let height = frame_buffer.resolution.vertical / CHAR_RESOLUTION.vertical;
        Self {
            frame_buffer,
            char_colors: CharColors::DEFAULT,
            cursor_position: CharPosition::initial(frame_buffer.resolution),
            width,
            height,
        }
    }

    pub(super) fn set_char_colors(&mut self, char_colors: CharColors) {
        self.char_colors = char_colors;
    }

    pub(super) fn put_char(&mut self, printable_char: PrintableChar) {
        self.draw_printable_char(printable_char);
        self.cursor_position.go_right_or_start_new_line();
        self.scroll_if_needed();
    }

    pub(super) fn go_down(&mut self) {
        self.cursor_position.go_down();
        self.scroll_if_needed();
    }
    pub(super) fn go_to_line_start(&mut self) {
        self.cursor_position.go_back_to_start_of_line();
        self.scroll_if_needed();
    }

    fn draw_printable_char(&mut self, printable_char: PrintableChar) {
        let mut pixel_position = self.cursor_position.pixel_position;
        for bit_line in CharBitMap::from(printable_char).get_lines() {
            for bit in bit_line.get_bits() {
                let pixel = self.char_colors.apply(bit);
                self.frame_buffer
                    .draw_pixel_if_visible(pixel_position, pixel);
                pixel_position.horizontal += 1;
            }
            pixel_position.horizontal = self.cursor_position.pixel_position.horizontal;
            pixel_position.vertical += 1;
        }
    }
    fn draw_char_if_some(&mut self, optional_char: OptionalColoredChar) {
        if optional_char.is_none() {
            return;
        }
        self.draw_printable_char(optional_char.printable_char);
    }

    fn get_pixel_position_moving_cursor(
        &mut self,
        row_index: usize,
        column_index: usize,
    ) -> PixelPosition {
        self.cursor_position
            .go_to_unchecked(row_index, column_index);
        self.cursor_position.pixel_position
    }
    fn copy_char(&mut self, dest: PixelPosition, src: PixelPosition) {
        for h_pixel_index in 0..CHAR_RESOLUTION.horizontal {
            for v_pixel_index in 0..CHAR_RESOLUTION.vertical {
                let src = PixelPosition {
                    horizontal: src.horizontal + h_pixel_index,
                    vertical: src.vertical + v_pixel_index,
                };
                let dest = PixelPosition {
                    horizontal: dest.horizontal + h_pixel_index,
                    vertical: dest.vertical + v_pixel_index,
                };
                self.frame_buffer.copy_one_pixel(dest, src);
            }
        }
    }
    fn clear_char(&mut self, pixel_position: PixelPosition) {
        for h_pixel_index in 0..CHAR_RESOLUTION.horizontal {
            for v_pixel_index in 0..CHAR_RESOLUTION.vertical {
                let pixel_position = PixelPosition {
                    horizontal: pixel_position.horizontal + h_pixel_index,
                    vertical: pixel_position.vertical + v_pixel_index,
                };
                self.frame_buffer
                    .draw_pixel_if_visible(pixel_position, Pixel::BLACK);
            }
        }
    }
    fn scroll_if_needed(&mut self) {
        if self
            .cursor_position
            .the_whole_char_can_be_drawn_vertically()
        {
            return;
        }
        for row_index in 1..self.height {
            // starting with the second row !
            for column_index in 0..self.width {
                let src = self.get_pixel_position_moving_cursor(row_index, column_index);
                let dest = self.get_pixel_position_moving_cursor(row_index - 1, column_index);
                self.copy_char(dest, src);
            }
        }
        // clear last row
        for column_index in 0..self.width {
            let last_row_index = self.height - 1;
            let pixel_position =
                self.get_pixel_position_moving_cursor(last_row_index, column_index);
            self.clear_char(pixel_position);
        }
    }
}
