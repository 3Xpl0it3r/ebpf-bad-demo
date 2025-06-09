#include "vmlinux.h"
#include "bpf_helpers.h"
#include "bpf_tracing.h"

#define ETH_P_IP 0x800
#define ETH_P_IPV6 0x86dd



SEC("kprobe/tcp_v4_rcv")
int kprobe__tcp_v4_rcv(struct pt_regs *ctx){
	struct sk_buff *skb = (struct sk_buff *)PT_REGS_PARM1(ctx);
	void *skb_head = {0}; 
	bpf_probe_read_kernel(&skb_head, sizeof(skb_head), (void *)skb + offsetof(struct sk_buff, head)); // <- 导致异常
    return 0;
}


char _license[] SEC("license") = "GPL";
