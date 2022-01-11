#![no_std]
#![no_main]

mod vga_printer;
mod logger;
use bootloader::{
    entry_point,
    BootInfo//,
    // binary::logger::Logger
};
use core::{panic::PanicInfo};
use logger::{init_logger};

entry_point!(kernel_main);

// static HELLO: &str = "Hello World!";

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        // let screen = framebuffer.buffer_mut(); 
        let fb_info = framebuffer.info();
        init_logger(framebuffer.buffer_mut(),fb_info);
        log::info!("Hello World!");
        log::error!("Houston we have a problem!")
        // let mut printer = vga_printer::VGAPrinter::new(framebuffer.buffer_mut(), fb_info);
        // let prompt = "Hello WORLD!";
        // printer.write_str(prompt).expect("Failed to write string");
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
