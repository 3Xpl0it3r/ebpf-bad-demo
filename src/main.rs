use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::BufRead;
use std::mem::{self, MaybeUninit};
use std::ptr;
use std::sync::LazyLock;
use std::time::Duration;

use anyhow::{anyhow, Result};
use chrono::offset::MappedLocalTime;
use chrono::prelude::*;
use tokio::runtime::Runtime;

use bollard::Docker;
use clap::Parser;
use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};
use libbpf_rs::{libbpf_sys, MapCore, PerfBuffer, PerfBufferBuilder};
use libc::{c_char, htonl, htons, ntohl, ntohs};
use plain::Plain;

mod demo {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/bpf/demo.skel.rs"
    ));
}

use demo::*;


static cus_btf_path: &str = "/sys/kernel/btf/vmlinux";

fn build_custom_btf_open_opt(btf_custom_path: &str) -> libbpf_sys::bpf_object_open_opts {
    let _path = CString::new(btf_custom_path).unwrap();
    let cus_btf_fd: *const ::std::os::raw::c_char = _path.into_raw();

    let mut opts = libbpf_sys::bpf_object_open_opts {
        sz: mem::size_of::<libbpf_sys::bpf_object_open_opts>() as libbpf_sys::size_t,
        object_name: ptr::null(),
        relaxed_maps: false,
        pin_root_path: ptr::null(),
        kconfig: ptr::null(),
        btf_custom_path: ptr::null(),
        kernel_log_buf: ptr::null_mut(),
        kernel_log_size: 0,
        kernel_log_level: 0,
        ..Default::default()
    };

    opts.btf_custom_path = cus_btf_fd;
    opts
}

fn main() {
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        println!("remove limit on locked memory failed, ret is {}", ret);
    }
    /* let btf_custom_path = "/sys/kernel/btf/vmlinux"; */
    let mut skel_builder = DemoSkelBuilder::default();

    // set constants
    let mut open_project = MaybeUninit::uninit();
    let open_skel = skel_builder
        .open_opts(
            build_custom_btf_open_opt("/root/pahole/build/vmlinux"),
            &mut open_project,
        )
        .unwrap();

    // load ebpf code into kernel
    let mut skel = open_skel.load().unwrap();
    // let _ = skel.attach();



}
