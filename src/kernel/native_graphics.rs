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
pub(crate) struct FrameBuffer {
    mut_ptr_to_pixels: *mut HardwarePixel,
    pixel_format: HardwarePixelFormat,
    /// may be larger than horizontal_resolution, for performance reasons, or due to hardware restrictions !
    hardware_size_in_bytes: usize,
    hardware_width_in_pixels: usize,
    horizontal_resolution: usize,
    vertical_resolution: usize,
}

impl FrameBuffer {
    pub(crate) fn from_uefi_graphics_output_protocol(mut frame_buffer: gop::FrameBuffer, mode_info: ModeInfo) -> Self {
        Self {
            // Safe : UEFI frame buffers must be 32*n-byte-sized for our supported pixel formats
            mut_ptr_to_pixels: unsafe { mem::transmute(frame_buffer.as_mut_ptr()) },
            pixel_format: HardwarePixelFormat::from_uefi_pixel_format(mode_info.pixel_format()),
            hardware_size_in_bytes: frame_buffer.size(),
            hardware_width_in_pixels: mode_info.stride(),
            horizontal_resolution: mode_info.resolution().0,
            vertical_resolution: mode_info.resolution().1,
        }
    }
    pub(crate) fn get_horizontal_resolution(&self) -> usize {
        self.horizontal_resolution
    }
    pub(crate) fn get_vertical_resolution(&self) -> usize {
        self.vertical_resolution
    }
    pub(crate) fn draw_pixel_if_visible(&mut self, x: usize, y: usize, pixel: Pixel) {
        let hardware_pixel = HardwarePixel::new(pixel, self.pixel_format);
        // Safe :
        // - This code is available after we got the hardware frame buffer without panicking, whose geometries are known
        // - Once the boot stage is over, we may keep writing into the frame buffer : our OS won't support other cases
        unsafe {
            self.mut_ptr_to_pixels
                .offset((y * self.hardware_width_in_pixels + x) as isize)
                .write_volatile(hardware_pixel);
        }
    }
    pub(crate) fn blacken(&mut self) {
        self.fill(Pixel::BLACK);
    }
    pub(crate) fn fill(&mut self, pixel: Pixel) {
        for x in 0..self.horizontal_resolution {
            for y in 0..self.vertical_resolution {
                self.draw_pixel_if_visible(x, y, pixel);
            }
        }
    }
}
