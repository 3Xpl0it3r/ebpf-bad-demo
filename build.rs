use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{env, fs};

use libbpf_cargo::SkeletonBuilder;

const SRC: &str = "src/bpf/demo.bpf.c";

fn os_support_btf() -> bool {
    Path::new("/sys/kernel/btf/vmlinux").exists()
}

fn os_kernel_version() -> (u32, u32, u32) {
    let version_str = fs::read_to_string("/proc/version")
        .ok()
        .and_then(|s| s.split_whitespace().nth(2).map(|v| v.to_string()))
        .expect("Cannot get kernel version");
    let parts: Vec<u32> = version_str
        .split('.')
        .take(3) // 只取前 3 部分（主版本、次版本、修订号）
        .filter_map(|s| s.parse().ok())
        .collect();

    (parts[0], parts[1], parts[2])
}

fn main() {
    let out = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set in build script"),
    )
    .join("src")
    .join("bpf")
    .join("demo.skel.rs");

    let arch = env::var_os("CARGO_CFG_TARGET_ARCH")
        .expect("CARGO_CFG_TARGET_ARCH must be set in build script");

    let mut skeleton_builder = SkeletonBuilder::new();
    skeleton_builder.source(SRC);
    let mut build_args = vec![
        OsStr::new("-I").to_owned(),
        PathBuf::from("src/bpf/libbpf/src").as_os_str().to_owned(),
        OsStr::new("-I").to_owned(),
        PathBuf::from("src/bpf/include").as_os_str().to_owned(),
    ];

        // 特殊的内核版本
            build_args.append(&mut vec![
                OsStr::new("-I").to_owned(),
                PathBuf::from("src/bpf/headers")
                    .as_os_str()
                    .to_owned(),
            ]);
    skeleton_builder
        .clang_args(build_args)
        .build_and_generate(out)
        .unwrap();

    println!("cargo:rerun-if-changed={SRC}");
}
