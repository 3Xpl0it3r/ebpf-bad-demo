## 运行环境
```bash
# 操作系统版本
[root@kvm workspace]# uname -r 
4.19.90-2107.6.0.0100.oe1.bclinux.x86_64
[root@kvm workspace]# cat /etc/redhat-release 
BigCloud Enterprise Linux For Euler release 21.10 (LTS-SP2)
[root@kvm workspace]#

## 编译选项
[root@kvm workspace]# grep "CONFIG_BPF" /boot/config-$(uname -r)
CONFIG_BPF=y
CONFIG_BPF_SYSCALL=y
CONFIG_BPF_JIT_ALWAYS_ON=y
# CONFIG_BPFILTER is not set
CONFIG_BPF_JIT=y
CONFIG_BPF_STREAM_PARSER=y
CONFIG_BPF_EVENTS=y
# CONFIG_BPF_KPROBE_OVERRIDE is not set
[root@kvm workspace]#

## clangd版本
[root@kvm workspace]# clang --version
clang version 10.0.1 (openEuler 10.0.1-4.oe1 ecab2ef6cfc44c32848acde7525048ade42ebe97)
Target: x86_64-unknown-linux-gnu
Thread model: posix
InstalledDir: /usr/bin
[root@kvm workspace]#

## Rust版本
[root@kvm workspace]# rustc  --version
rustc 1.87.0 (17067e9ac 2025-05-09)
[root@kvm workspace]#
```

## 自定义BTF
```c
yum install kernel-debuginfo.x86_64
rpm -ql kernel-debuginfo.x86_64 |grep vmlinux

cp /usr/lib/debug/lib/modules/4.19.90-2107.6.0.0251.88.oe1.bclinux.x86_64/vmlinux vmlinux
pahole -J  vmlinux

vim src/main.rs
---
  let open_skel = skel_builder
        .open_opts(
            build_custom_btf_open_opt("/root/pahole/build/vmlinux"),
            &mut open_project,
        )
        .unwrap();
---
```


## 构建
```bash
cargo build 
./target/debug/demo
```

## 报错信息
```c
[root@kvm demo]# ./target/debug/demo 
libbpf: BTF loading error: -22
libbpf: -- BEGIN BTF LOAD LOG ---
magic: 0xeb9f
version: 1
flags: 0x0
hdr_len: 24
type_off: 0
type_len: 2432
str_off: 2432
str_len: 1792
btf_total_size: 4248
[1] PTR (anon) type_id=2
[2] STRUCT pt_regs size=168 vlen=21
	r15 type_id=3 bits_offset=0
	r14 type_id=3 bits_offset=64
	r13 type_id=3 bits_offset=128
	r12 type_id=3 bits_offset=192
	bp type_id=3 bits_offset=256
	bx type_id=3 bits_offset=320
	r11 type_id=3 bits_offset=384
	r10 type_id=3 bits_offset=448
	r9 type_id=3 bits_offset=512
	r8 type_id=3 bits_offset=576
	ax type_id=3 bits_offset=640
	cx type_id=3 bits_offset=704
	dx type_id=3 bits_offset=768
	si type_id=3 bits_offset=832
	di type_id=3 bits_offset=896
	orig_ax type_id=3 bits_offset=960
	ip type_id=3 bits_offset=1024
	cs type_id=3 bits_offset=1088
	flags type_id=3 bits_offset=1152
	sp type_id=3 bits_offset=1216
	ss type_id=3 bits_offset=1280
[3] INT long unsigned int size=8 bits_offset=0 nr_bits=64 encoding=(none)
[4] ENUM (anon) size=4 vlen=1
	ctx val=1
[5] INT int size=4 bits_offset=0 nr_bits=32 encoding=SIGNED
[6] TYPEDEF kprobe__tcp_v4_rcv type_id=4
[7] Invalid btf_info:84000050
-- END BTF LOAD LOG --
libbpf: Error loading .BTF into kernel: -22. BTF is optional, ignoring.
```

