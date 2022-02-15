#![no_std]
#![no_main]

mod logger;
mod vga_printer;
use bootloader::{entry_point, BootInfo, boot_info::{MemoryRegion, MemoryRegions}};
use core::{panic::PanicInfo, fmt::Result, ops::Deref};
use logger::init_logger;

const PANIC_HEADER: &str =
    "-------------------- Kernel Panic --------------------";

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // panic when framebuffer info couldn't be found
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();

    let fb_info = framebuffer.info();
    init_logger(framebuffer.buffer_mut(), fb_info);

    log::info!("Hello World!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    log::error!("{PANIC_HEADER}");
    log::error!("{_info}");
    loop {}
}
