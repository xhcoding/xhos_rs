#![no_std]
#![no_main]

use core::panic::PanicInfo;
use noto_sans_mono_bitmap::{FontWeight, RasterHeight};

mod framebuffer;
use framebuffer::{Font, WRITER};
mod macros;

// panic 时调用这个函数
// ! 表示这个函数是一个发散函数，从不返回
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn init_framebuffer(bootinfo: &'static mut bootloader_api::BootInfo) {
    let font = Font::new(RasterHeight::Size16, FontWeight::Regular);
    WRITER
        .lock()
        .init(bootinfo.framebuffer.as_mut().unwrap(), font);
}

bootloader_api::entry_point!(main);

pub fn main(bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    init_framebuffer(bootinfo);

    println!("Hello World!\nHello xhos!");
    panic!("This is panic")
}
