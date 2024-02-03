use crate::kernel::KernelContext;
use crate::uefi_boot::uefi_graphics::get_frame_buffer;
use uefi::table::boot::MemoryType;
use uefi::table::{Boot, SystemTable};

mod uefi_graphics;

pub(super) fn boot(system_table: SystemTable<Boot>) -> Option<KernelContext> {
    let boot_services = system_table.boot_services();
    let frame_buffer = get_frame_buffer(boot_services)?;
    let (system_table, memory_map) =
        system_table.exit_boot_services(MemoryType::custom(0xFFFFFFFF));
    let kernel_context = KernelContext {
        frame_buffer,
        system_table,
        memory_map,
    };
    Some(kernel_context)
}
