#![no_std]
#![no_main]

use bootloader::{
    entry_point,
    BootInfo//,
    // binary::logger::Logger
};
use core::{panic::PanicInfo, fmt::Write};

mod vga_printer;

entry_point!(kernel_main);

// static HELLO: &str = "Hello World!";

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // turn the screen gray
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {

        // let screen = framebuffer.buffer_mut(); 
        let fb_info = framebuffer.info();
        let mut printer = vga_printer::VGAPrinter::new(framebuffer.buffer_mut(), fb_info);
        let prompt = "Hello\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nDEAR\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nWORLD!";
        printer.write_str(prompt).expect("Failed to write string");
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
