#![no_std]
#![no_main]

use core::panic::PanicInfo;

// panic 时调用这个函数
// ! 表示这个函数是一个发散函数，从不返回
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(main);

pub fn main(bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = bootinfo.framebuffer.as_mut() {
        let mut value = 0x90;
        for byte in framebuffer.buffer_mut() {
            *byte = value;
            value = value.wrapping_add(2);
        }
    }

    loop {}
}
