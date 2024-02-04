#![no_main]
#![no_std]

mod kernel;
mod uefi_boot;

use crate::kernel::console::{Console, DisposablePanicWriter};
use crate::kernel::load;
use crate::uefi_boot::boot;
use core::panic::PanicInfo;
use uefi::table::{Boot, SystemTable};
use uefi::{entry, Handle, Status};

static mut PANIC_CONSOLE: Option<*mut Console> = None;

#[entry]
fn main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    let kernel_context = boot(system_table);
    if kernel_context.is_none() {
        return Status::UNSUPPORTED;
    }
    let mut kernel_context = kernel_context.unwrap();
    let mut console = Console::new(kernel_context.frame_buffer);
    unsafe {
        PANIC_CONSOLE = Some(&mut console);
    }
    load(&mut kernel_context, &mut console);
}

#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    if let Some(console) = PANIC_CONSOLE {
        DisposablePanicWriter::new(*console).panic(info);
    }
    loop {}
}
