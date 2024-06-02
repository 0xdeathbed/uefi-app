# uefi-app


```bash
cargo build --target x86_64-unknown-uefi -Z build-std=core,alloc \
    -Z build-std-features=compiler-builtins-mem
cargo run --package disk_image -- target/x86_64-unknown-uefi/debug/uefi-app.efi
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-unknown-uefi/debug/uefi_app.fat \
    -bios OVMF_pure-efi.fd
```
