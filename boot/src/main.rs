use bootloader::{BootConfig, UefiBoot};
use std::{env::args, path::Path, process::exit};

pub fn main() {
    let kernel_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let kernel_binary = kernel_dir.join(args().skip(1).next().unwrap());
    let uefi_image = kernel_dir.join("images").join(format!(
        "bootimage-uefi-{}.img",
        kernel_binary.file_stem().unwrap().to_str().unwrap()
    ));

    let mut config = BootConfig::default();
    config.serial_logging = false;

    let mut uefi = UefiBoot::new(&kernel_binary);

    uefi.set_boot_config(&config);

    if let Err(e) = uefi.create_disk_image(&uefi_image) {
        eprintln!("{:#?}", &e);
        exit(1)
    }
    println!("{}", uefi_image.display());
}
