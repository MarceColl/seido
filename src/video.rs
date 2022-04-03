use bootloader::boot_info::FrameBuffer;
use spin::Mutex;
use crate::serial_println;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn write_bgr(&self, mem: &mut [u8]) {
        mem[0] = self.b;
        mem[1] = self.g;
        mem[2] = self.r;
        mem[3] = 255;
    }
}

pub fn init_video(fb: &mut FrameBuffer) {
    let buffer_size = fb.info().byte_len;
    serial_println!("Framebuffer size {}", buffer_size);
    for byte in fb.buffer_mut() {
        // let value = if i % 4 == 2 {
        //     0xFF
        // } else {
        //     0x00
        // };

        *byte = 0x00;
    }
}

pub fn draw_pixel(fb: &mut FrameBuffer, x: usize, y: usize, color: &Color) {
    let fb_width = fb.info().horizontal_resolution;
    let bpp = fb.info().bytes_per_pixel;
    let pidx = fb_width * bpp * y + x * bpp;
    color.write_bgr(&mut fb.buffer_mut()[pidx..(pidx+4)]);
}