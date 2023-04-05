#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use regex::Regex;
use std::path::Path;
use std::process::{self, Command};

fn build_test_kernel() -> Vec<String> {
    let mut cmd = Command::new("cargo");
    let mut targets = Vec::new();
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
        for cap in re.captures_iter(stderr.as_str()) {
            targets.push(String::from(&cap[1]));
        }
        return targets;
    } else {
        panic!("Failed to build test kernel");
    }
}

fn build_run_kernel() -> Vec<String> {
    let mut cmd = Command::new("cargo");
    cmd.args([
        "build",
        "--package",
        "kernel",
        "--target",
        "x86_64-unknown-none",
    ]);

    cmd.output().expect("Failed to build test kernel");
    return vec![String::from("target/x86_64-unknown-none/debug/kernel")];
}

fn build_boot(targets: Vec<String>) -> Vec<String> {
    let mut paths = Vec::new();
    for target in targets {
        let mut cmd = Command::new("cargo");
        cmd.args(["run", "--package", "boot", target.as_str()]);

        paths.push(String::from(
            String::from_utf8(cmd.output().unwrap().stdout)
                .unwrap()
                .trim(),
        ));
    }
    return paths;
}

fn run_qemu(images: Vec<String>) {
    let kernel_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let ovme_image = kernel_dir.join("OVMF-pure-efi.fd");

    for image in images {
        let mut cmd = Command::new("qemu-system-x86_64");

        cmd.arg("-drive")
            .arg(format!("format=raw,file={}", image))
            .arg("-bios")
            .arg(&ovme_image)
            .arg("-device")
            .arg("isa-debug-exit,iobase=0xf4,iosize=0x04")
            .arg("-serial")
            .arg("stdio");

        if cfg!(test) {
            cmd.arg("-display").arg("none");
        }

        let status = cmd.status().unwrap();

        let code = status.code().unwrap();
        if code != 33 {
            process::exit(code);
        }
    }
    process::exit(0);
}

fn main() {
    let targets;
    if cfg!(test) {
        println!("Build test kernel");
        targets = build_test_kernel();
    } else {
        println!("Build run kernel");
        targets = build_run_kernel();
    }

    println!("Build bootimage for {:?}", targets);

    let paths = build_boot(targets);

    println!("Run images: {:?}", paths);
    run_qemu(paths);
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    main();
}
