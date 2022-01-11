use core::{fmt};

use bootloader::boot_info::{FrameBufferInfo,PixelFormat};
use font8x8::UnicodeFonts;
use rgb::RGB8;

const LINE_SPACING: usize = 0;
const CHAR_HEIGHT: usize = 8;
pub const DEFAULT_COLOR: RGB8 = RGB8{r:255,g:255,b:255}; // white

pub struct VGAPrinter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
    color: RGB8,
}

impl VGAPrinter{
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut printer = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
            color: DEFAULT_COLOR,
        };
        printer.clear();
        printer
    }

    pub fn clear(&mut self) {
        self.x_pos = 0;
        self.y_pos = 0;
        self.framebuffer.fill(0)
    }

    pub fn add_vspace(&mut self, space: usize) {
        self.y_pos += space;
    }

    pub fn set_color(&mut self, color: RGB8) {
        self.color = color
    }

    fn width(&self) -> usize {
        self.info.horizontal_resolution
    }

    fn height(&self) -> usize {
        self.info.vertical_resolution
    }

    fn carriage_return(&mut self) {
        self.x_pos = 0;
    }

    fn newline(&mut self) {
        if self.y_pos >= (self.height() - CHAR_HEIGHT) {
            // screen overflow
            self.clear();
            return
        }
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
        let color = VGAPrinter::new_color_with_intensity(self.color,intensity);

        let pixel_color = match self.info.pixel_format {
            PixelFormat::RGB => [
                
                color.r,
                color.g,
                color.b,
                0],
            PixelFormat::BGR => [
                color.b,
                color.g,
                color.r,
                0],
            PixelFormat::U8  => [
                VGAPrinter::rgb_to_grayscale(color),
                 0,
                 0,
                 0],
            _ => panic!()
        };
        
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset+bytes_per_pixel)]
            .copy_from_slice(&pixel_color[..bytes_per_pixel]);
    }
    
    fn rgb_to_grayscale(color: RGB8) -> u8 {
        // magic numbers taken from
        // https://www.dynamsoft.com/blog/insights/image-processing/image-processing-101-color-space-conversion/
        (0.299 * f64::from(color.r)
        + 0.587 * f64::from(color.g)
        + 0.114 * f64::from(color.b)) as u8
    }

    fn new_color_with_intensity(color: RGB8, intensity: u8) -> RGB8 {
        let intense_factor = f64::from(intensity)/f64::from(u8::MAX);
        RGB8 {
            r: (f64::from(color.r)*intense_factor) as u8,
            g: (f64::from(color.g)*intense_factor) as u8,
            b: (f64::from(color.b)*intense_factor) as u8}
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
