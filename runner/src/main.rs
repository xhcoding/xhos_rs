use std::path::Path;
use std::process::Command;

fn main() {
    Command::new("cargo")
        .arg("run")
        .arg("--package")
        .arg("boot")
        .status()
        .unwrap();

    let kernel_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let uefi_image = kernel_dir.join("bootimage-uefi-xhos.img");

    let status = Command::new("qemu-system-x86_64")
        .arg("-drive")
        .arg(format!("format=raw,file={}", uefi_image.display()))
        .arg("-bios")
        .arg("OVMF-pure-efi.fd")
        .status()
        .unwrap();
    println!("Status: {}", status);
}
