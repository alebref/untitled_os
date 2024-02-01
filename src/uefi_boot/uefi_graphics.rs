use crate::kernel::native_graphics::{FrameBuffer, Resolution};
use core::cmp::Ordering;
use uefi::prelude::BootServices;
use uefi::proto::console::gop;
use uefi::proto::console::gop::{GraphicsOutput, Mode};
use uefi::table::boot::ScopedProtocol;

pub(super) fn get_frame_buffer(boot_services: &BootServices) -> Option<FrameBuffer> {
    let graphics_output_handle = boot_services
        .get_handle_for_protocol::<GraphicsOutput>()
        .ok()?;
    let mut graphics_output_protocol = boot_services
        .open_protocol_exclusive::<GraphicsOutput>(graphics_output_handle)
        .ok()?;
    let mode = select_highest_supported_mode(&mut graphics_output_protocol, boot_services)?;
    graphics_output_protocol.set_mode(&mode).ok()?;
    let frame_buffer = graphics_output_protocol.frame_buffer();
    let mode_info = mode.info();
    Some(FrameBuffer::from_uefi_graphics_output_protocol(
        frame_buffer,
        *mode_info,
    ))
}

fn select_highest_supported_mode(
    graphics_output_protocol: &mut ScopedProtocol<'_, GraphicsOutput>,
    boot_services: &BootServices,
) -> Option<Mode> {
    graphics_output_protocol
        .modes(boot_services)
        .filter(is_supported)
        .max_by(compare_horizontal_resolutions)
}

fn supports_32bit_pixels_direct_drawing(mode: &Mode) -> bool {
    matches!(
        mode.info().pixel_format(),
        gop::PixelFormat::Bgr | gop::PixelFormat::Rgb
    )
}

fn has_supported_resolution(mode: &Mode) -> bool {
    Resolution {
        horizontal: mode.info().resolution().0,
        vertical: mode.info().resolution().1,
    }
    .is_supported()
}

fn is_supported(mode: &Mode) -> bool {
    supports_32bit_pixels_direct_drawing(mode) && has_supported_resolution(mode)
}

fn get_horizontal_resolution(mode: &Mode) -> usize {
    mode.info().resolution().0
}

fn compare_horizontal_resolutions(mode1: &Mode, mode2: &Mode) -> Ordering {
    get_horizontal_resolution(mode1).cmp(&get_horizontal_resolution(mode2))
}
