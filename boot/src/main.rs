use bootloader::UefiBoot;
use std::{path::Path, process::exit};

pub fn main() {
    // 内核二进制
    let kernel_binary = Path::new(env!("CARGO_BIN_FILE_KERNEL_kernel"));

    // 镜像生成的目录为根目录
    let kernel_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let uefi_image = kernel_dir.join("bootimage-uefi-xhos.img");

    // UEFI 启动镜像生成器
    let uefi = UefiBoot::new(&kernel_binary);

    if let Err(e) = uefi.create_disk_image(&uefi_image) {
        eprintln!("{:#?}", &e);
        exit(1)
    }
}
