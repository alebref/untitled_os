use core::mem;
use uefi::proto::console::gop;
use uefi::proto::console::gop::{ModeInfo, PixelFormat};

#[derive(Clone, Copy, Debug)]
enum HardwarePixelFormat {
    Bgr,
    Rgb,
}

impl HardwarePixelFormat {
    /// # Panics
    /// Panics if `pixel_format` doesn't match `PixelFormat::Bgr | PixelFormat::Rgb`
    fn from_uefi_pixel_format(pixel_format: PixelFormat) -> Self {
        match pixel_format {
            PixelFormat::Bgr => HardwarePixelFormat::Bgr,
            PixelFormat::Rgb => HardwarePixelFormat::Rgb,
            _ => unreachable!("Only UEFI PixelRedGreenBlueReserved8BitPerColor and PixelBlueGreenRedReserved8BitPerColor pixel formats are supported and should have been selected before")
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct HardwarePixel(u32);

impl HardwarePixel {
    const fn new(Pixel { red, green, blue }: Pixel, pixel_format: HardwarePixelFormat) -> Self {
        match pixel_format {
            HardwarePixelFormat::Bgr => Self( u32::from_le_bytes([blue, green, red, 0])),
            _ => Self( u32::from_le_bytes([red, green, blue, 0]))
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel {
    pub(crate) const BLACK: Self = Self {
        red: 0,
        green: 0,
        blue: 0,
    };
    pub(crate) const RED: Self = Self {
        red: 255,
        green: 0,
        blue: 0,
    };
    pub(crate) const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Position {
    pub(crate) horizontal: usize,
    pub(crate) vertical: usize,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Resolution {
    pub(crate) horizontal: usize,
    pub(crate) vertical: usize,
}

impl Resolution {
    const MIN_SUPPORTED: Self = Self {
        horizontal: 320,
        vertical: 200,
    };
    const MAX_SUPPORTED: Self = Self {
        horizontal: 1920,
        vertical: 1080,
    };
    pub(crate) const fn is_supported(other: Self) -> bool {
        other.horizontal >= Self::MIN_SUPPORTED.horizontal
            && other.horizontal <= Self::MAX_SUPPORTED.horizontal
            && other.vertical >= Self::MIN_SUPPORTED.vertical
            && other.vertical <= Self::MAX_SUPPORTED.vertical
    }
    pub(crate) const fn accepts_position(&self, position: Position) -> bool {
        position.horizontal < self.horizontal
            && position.horizontal < self.horizontal
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct FrameBuffer {
    mut_ptr_to_pixels: *mut HardwarePixel,
    pixel_format: HardwarePixelFormat,
    hardware_size_in_bytes: usize,
    /// may be larger than horizontal_resolution, for performance reasons, or due to hardware restrictions !
    hardware_width_in_pixels: usize,
    pub(crate) resolution: Resolution,
}

impl FrameBuffer {
    pub(crate) fn from_uefi_graphics_output_protocol(mut frame_buffer: gop::FrameBuffer, mode_info: ModeInfo) -> Self {
        Self {
            // Safe : UEFI frame buffers must be 32*n-byte-sized for our supported pixel formats
            mut_ptr_to_pixels: unsafe { mem::transmute(frame_buffer.as_mut_ptr()) },
            pixel_format: HardwarePixelFormat::from_uefi_pixel_format(mode_info.pixel_format()),
            hardware_size_in_bytes: frame_buffer.size(),
            hardware_width_in_pixels: mode_info.stride(),
            resolution: Resolution {
                horizontal: mode_info.resolution().0,
                vertical: mode_info.resolution().1,
            },
        }
    }
    pub(crate) fn draw_pixel_if_visible(&mut self, position: Position, pixel: Pixel) {
        if !self.resolution.accepts_position(position) {
            return;
        }
        // Safe :
        // - This code is available after we got the hardware frame buffer without panicking, whose geometries are known
        // - Once the boot stage is over, we may keep writing into the frame buffer : our OS won't support other cases
        // - We just have validated x and y
        unsafe { self.draw_pixel_unchecked(position, pixel); }
    }
    unsafe fn draw_pixel_unchecked(&mut self, position: Position, pixel: Pixel) {
        let hardware_pixel = HardwarePixel::new(pixel, self.pixel_format);
        unsafe {
            self.mut_ptr_to_pixels
                .offset((position.vertical * self.hardware_width_in_pixels + position.horizontal) as isize)
                .write_volatile(hardware_pixel);
        }
    }
    pub(crate) fn blacken(&mut self) {
        self.fill(Pixel::BLACK);
    }
    pub(crate) fn fill(&mut self, pixel: Pixel) {
        for horizontal in 0..self.resolution.horizontal {
            for vertical in 0..self.resolution.vertical {
                let position = Position {
                    horizontal,
                    vertical,
                };
                // Safe : position comes from self.resolution
                unsafe { self.draw_pixel_unchecked(position, pixel); }
            }
        }
    }
}
