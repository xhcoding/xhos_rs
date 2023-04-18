#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use noto_sans_mono_bitmap::{FontWeight, RasterHeight};

pub mod framebuffer;
pub mod serial;
pub mod interrupts;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...  ", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    serial_println!("Running {} tests finished", tests.len());
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
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

fn init_framebuffer(bootinfo: &'static mut bootloader_api::BootInfo) {
    use crate::framebuffer::{Font, WRITER};

    let font = Font::new(RasterHeight::Size16, FontWeight::Regular);
    WRITER
        .lock()
        .init(bootinfo.framebuffer.as_mut().unwrap(), font);
}

pub fn init(bootinfo: &'static mut bootloader_api::BootInfo) {
    init_framebuffer(bootinfo);
    interrupts::init_idt();
}

#[cfg(test)]
bootloader_api::entry_point!(main);

#[cfg(test)]
pub fn main(bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    init(bootinfo);
    println!("This is test");
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
