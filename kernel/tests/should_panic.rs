#![no_std]
#![no_main]

use core::panic::PanicInfo;

use kernel::{exit_qemu, init, serial_println, QemuExitCode};

bootloader_api::entry_point!(main);

pub fn main(bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    init(bootinfo);

    should_fail();

    serial_println!("[test did not panic]");

    exit_qemu(QemuExitCode::Failed);

    loop {}
}

fn should_fail() {
    serial_println!("should fail...");
    assert_eq!(1, 0);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
