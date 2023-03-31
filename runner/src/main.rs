#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use regex::Regex;
use std::path::Path;
use std::process::Command;

fn build_test_kernel() -> Option<String> {
    let mut cmd = Command::new("cargo");

    cmd.args([
        "test",
        "--package",
        "kernel",
        "--target",
        "x86_64-unknown-none",
        "--no-run",
    ]);

    let output = cmd.output().expect("Failed to build test kernel");

    if output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("Get stderr failed");

        let re = Regex::new(r".*(target[\\/].*)\).*").unwrap();
        let mut target: Option<String> = None;
        for cap in re.captures_iter(stderr.as_str()) {
            target = Some(String::from(&cap[1]));
        }
        return target;
    } else {
        panic!("Failed to build test kernel");
    }
}

fn build_run_kernel() -> Option<String> {
    let mut cmd = Command::new("cargo");
    cmd.args([
        "build",
        "--package",
        "kernel",
        "--target",
        "x86_64-unknown-none",
    ]);

    cmd.output().expect("Failed to build test kernel");
    return Some(String::from("target/x86_64-unknown-none/debug/kernel"));
}

fn build_boot(target: String) {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--package", "boot", target.as_str()]);

    cmd.output().unwrap();
}

fn run_qemu() {
    let kernel_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let uefi_image = kernel_dir.join("bootimage-uefi-xhos.img");
    let ovme_image = kernel_dir.join("OVMF-pure-efi.fd");

    let status = Command::new("qemu-system-x86_64")
        .arg("-drive")
        .arg(format!("format=raw,file={}", uefi_image.display()))
        .arg("-bios")
        .arg(ovme_image)
        .status()
        .unwrap();
    println!("Status: {}", status);
}

fn main() {
    let target;
    if cfg!(test) {
        println!("Build test kernel");
        target = build_test_kernel();
    } else {
        println!("Build run kernel");
        target = build_run_kernel();
    }

    println!("Build bootimage");

    build_boot(target.unwrap());

    println!("Run qemu");
    run_qemu();
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    main();
}
