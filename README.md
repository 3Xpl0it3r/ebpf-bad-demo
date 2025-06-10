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
[root@kvm demo]# strace  ./target/debug/demo
execve("./target/debug/demo", ["./target/debug/demo"], 0x7ffe570c9fd0 /* 25 vars */) = 0
brk(NULL)                               = 0x565174ab0000
access("/etc/ld.so.preload", R_OK)      = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
fstat(3, {st_mode=S_IFREG|0644, st_size=49572, ...}) = 0
mmap(NULL, 49572, PROT_READ, MAP_PRIVATE, 3, 0) = 0x7fdc8163b000
close(3)                                = 0
openat(AT_FDCWD, "/lib64/libelf.so.1", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0`3\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=100464, ...}) = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fdc81639000
mmap(NULL, 102424, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7fdc8161f000
mmap(0x7fdc81622000, 65536, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x3000) = 0x7fdc81622000
mmap(0x7fdc81632000, 20480, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x13000) = 0x7fdc81632000
mmap(0x7fdc81637000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x17000) = 0x7fdc81637000
close(3)                                = 0
openat(AT_FDCWD, "/lib64/libz.so.1", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0\0203\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=100392, ...}) = 0
mmap(NULL, 102416, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7fdc81605000
mprotect(0x7fdc81608000, 86016, PROT_NONE) = 0
mmap(0x7fdc81608000, 57344, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x3000) = 0x7fdc81608000
mmap(0x7fdc81616000, 24576, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x11000) = 0x7fdc81616000
mmap(0x7fdc8161d000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x17000) = 0x7fdc8161d000
close(3)                                = 0
openat(AT_FDCWD, "/lib64/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0\3402\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=96224, ...}) = 0
mmap(NULL, 99024, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7fdc815ec000
mmap(0x7fdc815ef000, 69632, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x3000) = 0x7fdc815ef000
mmap(0x7fdc81600000, 12288, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x14000) = 0x7fdc81600000
mmap(0x7fdc81603000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x16000) = 0x7fdc81603000
close(3)                                = 0
openat(AT_FDCWD, "/lib64/librt.so.1", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0\2203\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=39384, ...}) = 0
mmap(NULL, 43520, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7fdc815e1000
mprotect(0x7fdc815e4000, 24576, PROT_NONE) = 0
mmap(0x7fdc815e4000, 16384, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x3000) = 0x7fdc815e4000
mmap(0x7fdc815e8000, 4096, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x7000) = 0x7fdc815e8000
mmap(0x7fdc815ea000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x8000) = 0x7fdc815ea000
close(3)                                = 0
openat(AT_FDCWD, "/lib64/libpthread.so.0", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0@|\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=116328, ...}) = 0
mmap(NULL, 131552, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7fdc815c0000
mprotect(0x7fdc815c7000, 81920, PROT_NONE) = 0
mmap(0x7fdc815c7000, 61440, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x7000) = 0x7fdc815c7000
mmap(0x7fdc815d6000, 16384, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x16000) = 0x7fdc815d6000
mmap(0x7fdc815db000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1a000) = 0x7fdc815db000
mmap(0x7fdc815dd000, 12768, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7fdc815dd000
close(3)                                = 0
openat(AT_FDCWD, "/lib64/libm.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0P\362\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=1581928, ...}) = 0
mmap(NULL, 1581080, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7fdc8143d000
mmap(0x7fdc8144c000, 651264, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0xf000) = 0x7fdc8144c000
mmap(0x7fdc814eb000, 864256, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0xae000) = 0x7fdc814eb000
mmap(0x7fdc815be000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x180000) = 0x7fdc815be000
close(3)                                = 0
openat(AT_FDCWD, "/lib64/libdl.so.2", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0P\21\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=14424, ...}) = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fdc8143b000
mmap(NULL, 16528, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7fdc81436000
mmap(0x7fdc81437000, 4096, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1000) = 0x7fdc81437000
mmap(0x7fdc81438000, 4096, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x2000) = 0x7fdc81438000
mmap(0x7fdc81439000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x2000) = 0x7fdc81439000
close(3)                                = 0
openat(AT_FDCWD, "/lib64/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0@\\\2\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=1791192, ...}) = 0
mmap(NULL, 1799384, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7fdc8127e000
mprotect(0x7fdc812a3000, 1609728, PROT_NONE) = 0
mmap(0x7fdc812a3000, 1327104, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x25000) = 0x7fdc812a3000
mmap(0x7fdc813e7000, 278528, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x169000) = 0x7fdc813e7000
mmap(0x7fdc8142c000, 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1ad000) = 0x7fdc8142c000
mmap(0x7fdc81432000, 13528, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7fdc81432000
close(3)                                = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fdc8127c000
arch_prctl(ARCH_SET_FS, 0x7fdc8127cbc0) = 0
mprotect(0x7fdc8142c000, 12288, PROT_READ) = 0
mprotect(0x7fdc81439000, 4096, PROT_READ) = 0
mprotect(0x7fdc815be000, 4096, PROT_READ) = 0
mprotect(0x7fdc815db000, 4096, PROT_READ) = 0
mprotect(0x7fdc815ea000, 4096, PROT_READ) = 0
mprotect(0x7fdc81603000, 4096, PROT_READ) = 0
mprotect(0x7fdc8161d000, 4096, PROT_READ) = 0
mprotect(0x7fdc81637000, 4096, PROT_READ) = 0
mprotect(0x565173683000, 24576, PROT_READ) = 0
mprotect(0x7fdc8166e000, 4096, PROT_READ) = 0
munmap(0x7fdc8163b000, 49572)           = 0
set_tid_address(0x7fdc8127ce90)         = 4833
set_robust_list(0x7fdc8127cea0, 24)     = 0
rt_sigaction(SIGRTMIN, {sa_handler=0x7fdc815c7690, sa_mask=[], sa_flags=SA_RESTORER|SA_SIGINFO, sa_restorer=0x7fdc815d34c0}, NULL, 8) = 0
rt_sigaction(SIGRT_1, {sa_handler=0x7fdc815c7730, sa_mask=[], sa_flags=SA_RESTORER|SA_RESTART|SA_SIGINFO, sa_restorer=0x7fdc815d34c0}, NULL, 8) = 0
rt_sigprocmask(SIG_UNBLOCK, [RTMIN RT_1], NULL, 8) = 0
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
poll([{fd=0, events=0}, {fd=1, events=0}, {fd=2, events=0}], 3, 0) = 0 (Timeout)
rt_sigaction(SIGPIPE, {sa_handler=SIG_IGN, sa_mask=[PIPE], sa_flags=SA_RESTORER|SA_RESTART, sa_restorer=0x7fdc812b7690}, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
brk(NULL)                               = 0x565174ab0000
brk(0x565174ad1000)                     = 0x565174ad1000
brk(NULL)                               = 0x565174ad1000
openat(AT_FDCWD, "/proc/self/maps", O_RDONLY|O_CLOEXEC) = 3
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
fstat(3, {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
read(3, "5651735c6000-5651735d2000 r--p 0"..., 1024) = 1024
read(3, "8.so\n7fdc8142b000-7fdc8142c000 -"..., 1024) = 1024
read(3, "00 3805870                    /u"..., 1024) = 1024
read(3, "-7fdc815e4000 r--p 00000000 fd:0"..., 1024) = 1024
read(3, "o.1\n7fdc81604000-7fdc81605000 rw"..., 1024) = 1024
read(3, "7fdc81638000 r--p 00017000 fd:00"..., 1024) = 1024
close(3)                                = 0
sched_getaffinity(4833, 32, [0, 1])     = 8
rt_sigaction(SIGSEGV, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
sigaltstack(NULL, {ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=0}) = 0
mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0x7fdc81645000
mprotect(0x7fdc81645000, 4096, PROT_NONE) = 0
sigaltstack({ss_sp=0x7fdc81646000, ss_flags=0, ss_size=8192}, NULL) = 0
rt_sigaction(SIGSEGV, {sa_handler=0x56517363b890, sa_mask=[], sa_flags=SA_RESTORER|SA_ONSTACK|SA_SIGINFO, sa_restorer=0x7fdc815d34c0}, NULL, 8) = 0
rt_sigaction(SIGBUS, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGBUS, {sa_handler=0x56517363b890, sa_mask=[], sa_flags=SA_RESTORER|SA_ONSTACK|SA_SIGINFO, sa_restorer=0x7fdc815d34c0}, NULL, 8) = 0
prlimit64(0, RLIMIT_MEMLOCK, {rlim_cur=RLIM64_INFINITY, rlim_max=RLIM64_INFINITY}, NULL) = 0
faccessat(AT_FDCWD, "/proc/version_signature", R_OK) = -1 ENOENT (No such file or directory)
uname({sysname="Linux", nodename="kvm", ...}) = 0
openat(AT_FDCWD, "/sys/fs/bpf", O_RDONLY|O_DIRECTORY) = 3
bpf(0x24 /* BPF_??? */, 0x7ffd591702f0, 8) = -1 EINVAL (Invalid argument)
close(3)                                = 0
bpf(BPF_PROG_LOAD, {prog_type=BPF_PROG_TYPE_SOCKET_FILTER, insn_cnt=2, insns=0x7ffd59170110, license="GPL", log_level=0, log_size=0, log_buf=NULL, kern_version=KERNEL_VERSION(0, 0, 0), prog_flags=0, prog_name="", prog_ifindex=0, expected_attach_type=BPF_CGROUP_INET_INGRESS, prog_btf_fd=0, func_info_rec_size=0, func_info=NULL, func_info_cnt=0, line_info_rec_size=0, line_info=NULL, line_info_cnt=0, attach_btf_id=0, attach_prog_fd=0}, 148) = -1 EINVAL (Invalid argument)
prlimit64(0, RLIMIT_MEMLOCK, {rlim_cur=RLIM64_INFINITY, rlim_max=RLIM64_INFINITY}, NULL) = 0
bpf(BPF_PROG_LOAD, {prog_type=BPF_PROG_TYPE_SOCKET_FILTER, insn_cnt=2, insns=0x7ffd59170330, license="GPL", log_level=0, log_size=0, log_buf=NULL, kern_version=KERNEL_VERSION(0, 0, 0), prog_flags=0, prog_name="", prog_ifindex=0, expected_attach_type=BPF_CGROUP_INET_INGRESS, prog_btf_fd=0, func_info_rec_size=0, func_info=NULL, func_info_cnt=0, line_info_rec_size=0, line_info=NULL, line_info_cnt=0, attach_btf_id=0, attach_prog_fd=0}, 148) = 3
close(3)                                = 0
openat(AT_FDCWD, "/root/pahole/build/vmlinux", O_RDONLY|O_CLOEXEC) = 3
fstat(3, {st_mode=S_IFREG|0755, st_size=568049256, ...}) = 0
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\2\0>\0\1\0\0\0\0\0\0\1\0\0\0\0"..., 4096) = 4096
close(3)                                = 0
openat(AT_FDCWD, "/root/pahole/build/vmlinux", O_RDONLY|O_CLOEXEC) = 3
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
fstat(3, {st_mode=S_IFREG|0755, st_size=568049256, ...}) = 0
pread64(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\2\0>\0\1\0\0\0\0\0\0\1\0\0\0\0"..., 64, 0) = 64
pread64(3, "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"..., 5056, 568044200) = 5056
pread64(3, "\0.rela.altinstr_aux\0.debug_abbre"..., 688, 563630920) = 688
mmap(NULL, 4415488, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fdc80e46000
pread64(3, "\237\353\1\0\30\0\0\0\0\0\0\0\244\r'\0\244\r'\0\360F\34\0\1\0\0\0\0\0\0\1"..., 4412588, 563631608) = 4412588
mmap(NULL, 4415488, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fdc80a10000
brk(NULL)                               = 0x565174ad1000
brk(0x565174b0c000)                     = 0x565174b0c000
brk(NULL)                               = 0x565174b0c000
brk(NULL)                               = 0x565174b0c000
brk(0x565174af5000)                     = 0x565174af5000
brk(NULL)                               = 0x565174af5000
mmap(NULL, 290816, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fdc809c9000
brk(NULL)                               = 0x565174af5000
brk(NULL)                               = 0x565174af5000
brk(0x565174ad8000)                     = 0x565174ad8000
brk(NULL)                               = 0x565174ad8000
mremap(0x7fdc809c9000, 290816, 360448, MREMAP_MAYMOVE) = 0x7fdc80971000
mremap(0x7fdc80971000, 360448, 450560, MREMAP_MAYMOVE) = 0x7fdc80971000
munmap(0x7fdc80e46000, 4415488)         = 0
close(3)                                = 0
munmap(0x7fdc80a10000, 4415488)         = 0
munmap(0x7fdc80971000, 450560)          = 0
bpf(BPF_BTF_LOAD, {btf="\237\353\1\0\30\0\0\0\0\0\0\0000\0\0\0000\0\0\0\t\0\0\0\1\0\0\0\0\0\0\1"..., btf_log_buf=NULL, btf_size=81, btf_log_size=0, btf_log_level=0}, 40) = -1 EINVAL (Invalid argument)
bpf(BPF_BTF_LOAD, {btf="\237\353\1\0\30\0\0\0\0\0\0\0000\0\0\0000\0\0\0\5\0\0\0\0\0\0\0\0\0\0\1"..., btf_log_buf=NULL, btf_size=77, btf_log_size=0, btf_log_level=0}, 40) = -1 EINVAL (Invalid argument)
bpf(BPF_BTF_LOAD, {btf="\237\353\1\0\30\0\0\0\0\0\0\0\20\0\0\0\20\0\0\0\5\0\0\0\1\0\0\0\0\0\0\1"..., btf_log_buf=NULL, btf_size=45, btf_log_size=0, btf_log_level=0}, 40) = 3
close(3)                                = 0
bpf(BPF_BTF_LOAD, {btf="\237\353\1\0\30\0\0\0\0\0\0\0000\0\0\0000\0\0\0\t\0\0\0\1\0\0\0\0\0\0\1"..., btf_log_buf=NULL, btf_size=81, btf_log_size=0, btf_log_level=0}, 40) = -1 EINVAL (Invalid argument)
bpf(BPF_BTF_LOAD, {btf="\237\353\1\0\30\0\0\0\0\0\0\08\0\0\08\0\0\0\t\0\0\0\0\0\0\0\0\0\0\1"..., btf_log_buf=NULL, btf_size=89, btf_log_size=0, btf_log_level=0}, 40) = -1 EINVAL (Invalid argument)
bpf(BPF_BTF_LOAD, {btf="\237\353\1\0\30\0\0\0\0\0\0\0\f\0\0\0\f\0\0\0\7\0\0\0\1\0\0\0\0\0\0\20"..., btf_log_buf=NULL, btf_size=43, btf_log_size=0, btf_log_level=0}, 40) = -1 EINVAL (Invalid argument)
bpf(BPF_BTF_LOAD, {btf="\237\353\1\0\30\0\0\0\0\0\0\0(\0\0\0(\0\0\0\5\0\0\0\0\0\0\0\0\0\0\1"..., btf_log_buf=NULL, btf_size=69, btf_log_size=0, btf_log_level=0}, 40) = -1 EINVAL (Invalid argument)
bpf(BPF_BTF_LOAD, {btf="\237\353\1\0\30\0\0\0\0\0\0\0\f\0\0\0\f\0\0\0\10\0\0\0\1\0\0\0\0\0\0\23"..., btf_log_buf=NULL, btf_size=44, btf_log_size=0, btf_log_level=0}, 40) = -1 EINVAL (Invalid argument)
bpf(BPF_BTF_LOAD, {btf="\237\353\1\0\30\0\0\0\0\0\0\08\0\0\08\0\0\0\n\0\0\0\0\0\0\0\0\0\0\1"..., btf_log_buf=NULL, btf_size=90, btf_log_size=0, btf_log_level=0}, 40) = -1 EINVAL (Invalid argument)
bpf(BPF_BTF_LOAD, {btf="\237\353\1\0\30\0\0\0\0\0\0\0\200\t\0\0\200\t\0\0\0\7\0\0\0\0\0\0\0\0\0\2"..., btf_log_buf=NULL, btf_size=4248, btf_log_size=0, btf_log_level=0}, 40) = -1 EINVAL (Invalid argument)
mmap(NULL, 16781312, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fdc8027b000
bpf(BPF_BTF_LOAD, {btf="\237\353\1\0\30\0\0\0\0\0\0\0\200\t\0\0\200\t\0\0\0\7\0\0\0\0\0\0\0\0\0\2"..., btf_log_buf=0x7fdc8027b010, btf_size=4248, btf_log_size=16777215, btf_log_level=1}, 40) = -1 EINVAL (Invalid argument)
write(2, "libbpf: BTF loading error: -22\n", 31libbpf: BTF loading error: -22
) = 31
write(2, "libbpf: -- BEGIN BTF LOAD LOG --"..., 1134libbpf: -- BEGIN BTF LOAD LOG ---
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
) = 1134
munmap(0x7fdc8027b000, 16781312)        = 0
write(2, "libbpf: Error loading .BTF into "..., 72libbpf: Error loading .BTF into kernel: -22. BTF is optional, ignoring.
) = 72
bpf(BPF_PROG_LOAD, {prog_type=BPF_PROG_TYPE_TRACEPOINT, insn_cnt=6, insns=0x7ffd591702c0, license="GPL", log_level=0, log_size=0, log_buf=NULL, kern_version=KERNEL_VERSION(0, 0, 0), prog_flags=0, prog_name="", prog_ifindex=0, expected_attach_type=BPF_CGROUP_INET_INGRESS, prog_btf_fd=0, func_info_rec_size=0, func_info=NULL, func_info_cnt=0, line_info_rec_size=0, line_info=NULL, line_info_cnt=0, attach_btf_id=0, attach_prog_fd=0}, 148) = -1 EINVAL (Invalid argument)
bpf(BPF_PROG_LOAD, {prog_type=BPF_PROG_TYPE_SOCKET_FILTER, insn_cnt=2, insns=0x7ffd59170070, license="GPL", log_level=0, log_size=0, log_buf=NULL, kern_version=KERNEL_VERSION(0, 0, 0), prog_flags=0, prog_name="libbpf_nametest", prog_ifindex=0, expected_attach_type=BPF_CGROUP_INET_INGRESS, prog_btf_fd=0, func_info_rec_size=0, func_info=NULL, func_info_cnt=0, line_info_rec_size=0, line_info=NULL, line_info_cnt=0, attach_btf_id=0, attach_prog_fd=0}, 148) = 3
close(3)                                = 0
bpf(BPF_PROG_LOAD, {prog_type=BPF_PROG_TYPE_KPROBE, insn_cnt=11, insns=0x565174ab15d0, license="GPL", log_level=0, log_size=0, log_buf=NULL, kern_version=KERNEL_VERSION(4, 19, 90), prog_flags=0, prog_name="kprobe__tcp_v4_", prog_ifindex=0, expected_attach_type=BPF_CGROUP_INET_INGRESS, prog_btf_fd=0, func_info_rec_size=0, func_info=NULL, func_info_cnt=0, line_info_rec_size=0, line_info=NULL, line_info_cnt=0, attach_btf_id=0, attach_prog_fd=0}, 148) = 3
close(3)                                = 0
sigaltstack({ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=8192}, NULL) = 0
munmap(0x7fdc81645000, 12288)           = 0
exit_group(0)                           = ?
+++ exited with 0 +++
```

