use core::{ptr, fmt};

use bootloader::boot_info::{FrameBufferInfo,PixelFormat};
use font8x8::UnicodeFonts;

const LINE_SPACING: usize = 0;
const CHAR_HEIGHT: usize = 8;

pub struct VGAPrinter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
}

impl VGAPrinter{
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut printer = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
        };
        printer.clear();
        printer
    }

    fn width(&self) -> usize {
        self.info.horizontal_resolution
    }

    fn height(&self) -> usize {
        self.info.vertical_resolution
    }

    pub fn clear(&mut self) {
        self.x_pos = 0;
        self.y_pos = 0;
        self.framebuffer.fill(0)
    }

    fn carriage_return(&mut self) {
        self.x_pos = 0;
    }

    fn newline(&mut self) {
        self.y_pos = self.y_pos + CHAR_HEIGHT + LINE_SPACING;
        self.carriage_return();
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                if self.x_pos >= self.width(){
                    self.newline();
                }
                if self.y_pos >= (self.height() - CHAR_HEIGHT) {
                    self.clear()
                }
                let rendered_char = font8x8::BASIC_FONTS
                    .get(c)
                    .expect("character not found in basic font");
                self.write_rendered_char(rendered_char);
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: [u8;8]){
        for (y, byte) in rendered_char.iter().enumerate() {
            for (x, bit) in (0..8).enumerate() {
                let pixel = if *byte & (1 << bit) == 0 {0} else {255};
                self.write_pixel(self.x_pos + x, self.y_pos + y, pixel)
            }
        }
        self.x_pos += 8;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::RGB => [intensity, intensity, intensity/2, 0],
            PixelFormat::BGR => [intensity/2, intensity, intensity, 0],
            PixelFormat::U8  => [if intensity > 200 {0xf} else {0}, 0, 0, 0],
            _ => panic!()
        };
        
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset+bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe {ptr::read_volatile(&self.framebuffer[byte_offset])};
    }
}

impl fmt::Write for VGAPrinter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
