use uefi::table::{Boot, SystemTable};
use uefi::table::boot::MemoryType;
use crate::uefi_boot::uefi_graphics::get_frame_buffer;
use crate::kernel::KernelContext;
use crate::kernel::native_graphics::FrameBuffer;

mod uefi_graphics;

pub(super) struct BootResult {
    pub(super) frame_buffer: FrameBuffer,
    pub(super) kernel_context: KernelContext,
}

pub(super) fn boot(system_table: SystemTable<Boot>) -> Option<BootResult> {
    let boot_services = system_table.boot_services();
    let frame_buffer = get_frame_buffer(boot_services)?;
    let (system_table, memory_map) = system_table.exit_boot_services(MemoryType::custom(0xFFFFFFFF));
    let kernel_context = KernelContext {
        frame_buffer,
        system_table,
        memory_map,
    };
    Some(BootResult {
        frame_buffer,
        kernel_context,
    })
}
