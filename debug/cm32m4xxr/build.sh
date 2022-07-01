cargo build --example sche --release --all-features
riscv64-unknown-elf-objcopy -O binary ./target/riscv32imac-unknown-none-elf/release/examples/sche sche.bin