#![no_main]
#![no_std]

mod uefi_boot;
mod kernel;

use core::panic::PanicInfo;
use uefi::{entry, Handle, Status};
use uefi::table::{Boot, SystemTable};
use crate::kernel::load;
use crate::kernel::native_graphics::{FrameBuffer, Pixel};
use crate::uefi_boot::{boot, BootResult};

/// Made accessible to our `panic_handler` as a mutable static item, sorry...
static mut FRAME_BUFFER: Option<FrameBuffer> = None;

#[entry]
unsafe fn main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    let boot_result = boot(system_table);
    if boot_result.is_none() {
        return Status::UNSUPPORTED;
    }
    let BootResult { mut kernel_context, frame_buffer } = boot_result.unwrap();
    FRAME_BUFFER = Some(frame_buffer);
    load(&mut kernel_context);
}

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    if let Some(frame_buffer) = FRAME_BUFFER.as_mut() {
        frame_buffer.fill(Pixel::RED)
    }
    loop {}
}
