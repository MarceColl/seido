#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(seido::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::borrow::Borrow;
use core::panic::PanicInfo;
use bootloader::BootInfo;

use seido::{hlt_loop, println, serial_println};
use seido::video::init_video;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    seido::test_panic_handler(info);
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start(boot_info: &'static mut BootInfo) -> ! {
    serial_println!("Reached");
    serial_println!("Version: {}.{}.{}", boot_info.version_major, boot_info.version_minor, boot_info.version_patch);

    serial_println!("{:#?}", boot_info.framebuffer);

    seido::init();

    if let bootloader::boot_info::Optional::Some(framebuffer) = &mut boot_info.framebuffer {
        init_video(framebuffer);
    }

    println!("Welcome to this thing");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    // hlt_loop();
    hlt_loop();
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
