#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
#[cfg(test)]
use bootloader::{BootInfo, entry_point};
use x86_64::registers::segmentation::Segment;
use x86_64::structures::gdt::SegmentSelector;

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod video;
pub mod memory;
pub mod font_rendering;

pub fn init() {
    gdt::init();
    serial_println!("GDT Initialized");
    interrupts::init_idt();
    serial_println!("Interrupts Initialized");
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    serial_println!("Interrupts Enabled");
}

#[cfg(test)]
entry_point!(test_kmain);

#[cfg(test)]
fn test_kmain(_boot_info: &'static mut BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
    where
        T: Fn(),
{
    fn run(&self) {
        serial_print!("{}\t\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
