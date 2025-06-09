rm src/bpf/demo.skel.rs
rm -rf target/debug/build/demo-* target/debug/demo
# rm -rf target
cargo build 
sudo ./target/debug/demo

