/*!

framebuffer.rs 实现了向 framebuffer 输出字符串，framebuffer 是基于像素的 buffer ，
每个像素一般占用 4 个字节，前三个字节是颜色，根据格式可能是 RGB 或者 BGR 等等

*/

use bootloader_api::info::{FrameBuffer, PixelFormat};
use core::{fmt, ptr};
use noto_sans_mono_bitmap::{
    get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar,
};

pub struct Font {
    height: RasterHeight,
    width: usize,
    weight: FontWeight,
}

impl Font {
    pub fn new(raster_height: RasterHeight, font_weight: FontWeight) -> Self {
        Font {
            height: raster_height,
            width: get_raster_width(font_weight, raster_height),
            weight: font_weight,
        }
    }

    pub fn get_raster(&self, c: char) -> RasterizedChar {
        get_raster(c, self.weight, self.height).unwrap()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height.val()
    }
}

pub struct Writer {
    x: usize,
    y: usize,
    buffer: Option<&'static mut FrameBuffer>,
    font: Font,
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            x: 0,
            y: 0,
            buffer: None,
            font: Font::new(RasterHeight::Size16, FontWeight::Regular),
        }
    }

    pub fn init(&mut self, buffer: &'static mut FrameBuffer, font: Font) {
        self.buffer = Some(buffer);
        self.font = font;
        self.clear();
    }

    fn width(&self) -> usize {
        self.buffer.as_ref().unwrap().info().width
    }

    fn _height(&self) -> usize {
        self.buffer.as_ref().unwrap().info().height
    }

    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            _ => {
                let new_x = self.x + self.font.width();
                if new_x >= self.width() {
                    self.newline();
                }
                self.render_char(c)
            }
        }
    }

    fn newline(&mut self) {
        self.y += self.font.height();
        self.x = 0;
    }

    fn clear(&mut self) {
        self.buffer.as_mut().unwrap().buffer_mut().fill(0);
        self.x = 0;
        self.y = 0;
    }

    fn render_char(&mut self, c: char) {
        let char_raster = self.font.get_raster(c);
        for (y, row) in char_raster.raster().iter().enumerate() {
            for (x, intensity) in row.iter().enumerate() {
                self.render_pixel(self.x + x, self.y + y, *intensity);
            }
        }
        self.x += self.font.width();
    }

    fn render_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.buffer.as_mut().unwrap().info().stride + x;
        let color = match self.buffer.as_mut().unwrap().info().pixel_format {
            PixelFormat::Rgb => [intensity, intensity, intensity / 2, 0],
            PixelFormat::Bgr => [intensity / 2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xff } else { 0 }, 0, 0, 0],
            _ => panic!("pixel format not supported!"),
        };
        let bytes_per_pixel = self.buffer.as_mut().unwrap().info().bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;

        self.buffer.as_mut().unwrap().buffer_mut()[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..]);
        let _ =
            unsafe { ptr::read_volatile(&self.buffer.as_mut().unwrap().buffer_mut()[byte_offset]) };
    }
}

unsafe impl Send for Writer {}
unsafe impl Sync for Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
