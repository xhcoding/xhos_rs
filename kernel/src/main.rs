#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use noto_sans_mono_bitmap::{FontWeight, RasterHeight};

mod framebuffer;
use framebuffer::{Font, WRITER};
mod macros;
mod serial;

fn init_framebuffer(bootinfo: &'static mut bootloader_api::BootInfo) {
    let font = Font::new(RasterHeight::Size16, FontWeight::Regular);
    WRITER
        .lock()
        .init(bootinfo.framebuffer.as_mut().unwrap(), font);
}

bootloader_api::entry_point!(main);

pub fn main(bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    init_framebuffer(bootinfo);

    println!("Hello world, my name is xhos!");

    #[cfg(test)]
    test_main();

    loop {}
}

// panic 时调用这个函数
// ! 表示这个函数是一个发散函数，从不返回
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...  ", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    println!("Running {} tests finished", tests.len());
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
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
