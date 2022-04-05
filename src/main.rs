#![no_std]
#![no_main]

mod logger;
mod vga_printer;
use bootloader::{entry_point, BootInfo, boot_info::{MemoryRegions}};
use core::{panic::PanicInfo, ops::Deref};
use logger::init_logger;

const PANIC_HEADER: &str =
    "-------------------- Kernel Panic --------------------";

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let a = 5;
    // panic when framebuffer info couldn't be found
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();
    let fb_info = framebuffer.info();

    init_logger(framebuffer.buffer_mut(), fb_info);
    log::info!("Hello World!");

    check_memory(& boot_info.memory_regions);

    log::info!("a address: {:p}", &a);
    log::info!("kernel_main address: {:p}", &kernel_main);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    log::error!("{PANIC_HEADER}");
    log::error!("{_info}");
    loop {}
}

fn check_memory(memory_regions: &MemoryRegions) {
    for memory_region in memory_regions.deref() {
        // log::info!("{}", memory_region.start)
        log::info!("Detected memory region: [{:?}] 0x{:x} - 0x{:x} (0x{:x}b)", memory_region.kind, memory_region.start, memory_region.end, (memory_region.end - memory_region.start))
    }
}
