#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(seido::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point, boot_info::Optional};
use x86_64::structures::paging::{Page, Translate};
use x86_64::VirtAddr;

use seido::{hlt_loop, memory, println, serial_println};
use seido::memory::BootInfoFrameAllocator;
use seido::video::init_video;
use seido::font_rendering::{Glyph, render_text};

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    seido::test_panic_handler(info);
}

entry_point!(kmain);

fn kmain(boot_info: &'static mut BootInfo) -> ! {
    seido::init();

    let physical_memory_offset = if let Optional::Some(mem_offset) = boot_info.physical_memory_offset {
        mem_offset
    } else {
        panic!("No memory offset found");
    };

    let phys_mem_offset = VirtAddr::new(physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_regions)
    };

    serial_println!("{:?}", font8x8::HIRAGANA_UNICODE[1]);

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        serial_println!("{:?} -> {:?}", virt, phys);
    }

    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        use seido::video::Color;

        init_video(framebuffer);
        render_text(framebuffer, "hola com vas?", 30, 30, 1, &Color::new(255, 0, 0));
    }

    serial_println!("Welcome to this thing");

    #[cfg(test)]
    test_main();

    serial_println!("It did not crash!");
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
