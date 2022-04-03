use bootloader::boot_info::FrameBuffer;
use font8x8::UnicodeFonts;
use crate::serial_println;
use crate::video::Color;
use crate::video::draw_pixel;

#[repr(transparent)]
pub struct Glyph([u8; 8]);

impl Glyph {
    pub fn new(data: [u8; 8]) -> Self {
        Glyph(data)
    }

    pub fn sample(&self, x: usize, y: usize) -> bool {
        (self.0[y] & 1 << x) > 0
    }

    pub fn draw(&self, fb: &mut FrameBuffer, x: usize, y: usize, scale: usize, color: &Color) {
        for gx in 0..8 {
            for gy in 0..8 {
                for sx in 0..scale {
                    for sy in 0..scale {
                        if self.sample(gx, gy) {
                            draw_pixel(fb, x + (gx * scale) + sx, y + (gy * scale) + sy, color);
                        }
                    }
                }
            }
        }
    }
}

pub fn render_text(fb: &mut FrameBuffer, text: &str, x: usize, y: usize, scale: usize, color: &Color) {
    let mut dx = 0;
    for char in text.chars() {
        let glyph = Glyph::new(font8x8::BASIC_FONTS.get(char).unwrap());
        glyph.draw(fb, x + dx, y, scale, color);
        dx += 10 * scale;
    }
}