use crate::kernel::console::Console;
use crate::kernel::native_graphics::FrameBuffer;
use uefi::table::boot::MemoryMap;
use uefi::table::{Runtime, SystemTable};

pub(crate) mod console;
pub(crate) mod native_graphics;

#[derive(Debug)]
pub(crate) struct KernelContext {
    pub(crate) frame_buffer: FrameBuffer,
    pub(crate) system_table: SystemTable<Runtime>,
    pub(crate) memory_map: MemoryMap<'static>,
}

#[allow(unused_must_use)]
#[allow(unconditional_panic)]
pub(super) fn load(context: &mut KernelContext, console: &mut Console) -> ! {
    context.frame_buffer.blacken();

    for _ in 0..50 {
        console.print("\n");
    }

    console.print("Hello world !\nWelcome to Untitled OS :)\n\n");

    console.print("\t1. One\n");
    console.print("\t2. Two\n");
    console.print("\t3. Three...");
    console.print("\r\t3. Free !!!\n\n");

    console.print("The four next chars aren't printable and may be ignored : µéùà\n\n");

    console.print("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam id maximus leo, ut aliquam nulla. In hac habitasse platea dictumst. Pellentesque dictum egestas arcu vel ultricies. Sed pretium fermentum quam vitae molestie. Aenean vitae imperdiet ex. Quisque nec sagittis risus, quis accumsan ligula. Donec cursus convallis feugiat. Nullam eu velit odio. Donec blandit diam a erat venenatis, vitae tempor enim dapibus. Ut in pretium urna. Aenean convallis lacus at elit suscipit, vel posuere turpis hendrerit. Nulla lacus risus, tincidunt vitae ipsum vel, egestas tempus nisi. Nullam tincidunt eget risus at elementum. In iaculis, est sit amet congue volutpat, mauris arcu fringilla diam, a porttitor velit quam in magna.\n\n");

    console.print("Will it scroll ? (01/10)\n");
    console.print("Will it scroll ? (02/10)\n");
    console.print("Will it scroll ? (03/10)\n");
    console.print("Will it scroll ? (04/10)\n");
    console.print("Will it scroll ? (05/10)\n");
    console.print("Will it scroll ? (06/10)\n");
    console.print("Will it scroll ? (07/10)\n");
    console.print("Will it scroll ? (08/10)\n");
    console.print("Will it scroll ? (09/10)\n");
    console.print("Will it scroll ? (10/10)\n");
    console.print("Let's see...");

    // TODO check multiline panic printing
    //0 / 0;
    loop {}
}
