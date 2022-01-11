use crate::vga_printer::{VGAPrinter, DEFAULT_COLOR};
use conquer_once::spin::OnceCell;
use spinning_top::Spinlock;
use bootloader::boot_info::{FrameBufferInfo};
use core::fmt::Write;
use rgb::RGB8;
/// The global logger instance used for the `log` crate.
pub static LOGGER: OnceCell<Logger> = OnceCell::uninit();

/// A [`Logger`] instance protected by a spinlock.
pub struct Logger(Spinlock<VGAPrinter>);

const LOG_SPACING: usize = 2;

impl Logger {
    /// Create a new instance that logs to the given framebuffer.
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        Logger(Spinlock::new(VGAPrinter::new(framebuffer, info)))
    }

    /// Force-unlocks the logger to prevent a deadlock.
    ///
    /// This method is not memory safe and should be only used when absolutely necessary.
    pub unsafe fn force_unlock(&self) {
        unsafe { self.0.force_unlock() };
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let mut logger = self.0.lock();
        match record.level() {
            log::Level::Error => logger.set_color(RGB8{r: 225,g: 66,b: 66}),
            _ => logger.set_color(DEFAULT_COLOR)
        }
        writeln!(logger, "{}:    {}", record.level(), record.args()).unwrap();
        logger.add_vspace(LOG_SPACING);
    }

    fn flush(&self) {}
}

pub fn init_logger(framebuffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || Logger::new(framebuffer, info));
    log::set_logger(logger).expect("logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Framebuffer info: {:?}", info);
}
