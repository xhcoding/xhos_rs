#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kernel::{init_framebuffer, print, println};

bootloader_api::entry_point!(main);

pub fn main(bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    init_framebuffer(bootinfo);
    println!("Hello this is basic boot test.");
    #[cfg(test)]
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output")
}
