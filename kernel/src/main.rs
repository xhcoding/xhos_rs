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
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
