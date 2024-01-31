use crate::kernel::native_graphics::FrameBuffer;
use uefi::table::boot::MemoryMap;
use uefi::table::{Runtime, SystemTable};

pub(crate) mod native_graphics;

#[derive(Debug)]
pub(crate) struct KernelContext {
    pub(crate) frame_buffer: FrameBuffer,
    pub(crate) system_table: SystemTable<Runtime>,
    pub(crate) memory_map: MemoryMap<'static>,
}

pub(super) fn load(context: &mut KernelContext) -> ! {
    context.frame_buffer.blacken();
    panic!("lol");
}
