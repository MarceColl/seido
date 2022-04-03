// use bootloader::boot_info::FrameBuffer;
// use crate::serial_println;
//
// pub fn init_video(fb: &mut FrameBuffer) {
//     let buffer_size = fb.info().byte_len;
//     serial_println!("Framebuffer size {}", buffer_size);
//     for i in 0..(buffer_size*2) {
//         fb.buffer_mut()[i] = 255;
//     }
// }