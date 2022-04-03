#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(seido::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::{Page, Translate};
use x86_64::VirtAddr;

use seido::{hlt_loop, memory, println, serial_println};
use seido::memory::BootInfoFrameAllocator;
// use seido::video::init_video;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    seido::test_panic_handler(info);
}

entry_point!(kmain);

fn kmain(boot_info: &'static BootInfo) -> ! {

    seido::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    // if let bootloader::boot_info::Optional::Some(framebuffer) = &mut boot_info.framebuffer {
    //     init_video(framebuffer);
    // }

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
