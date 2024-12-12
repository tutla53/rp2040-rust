# Raspberry Pi Pico RP2040 + Embassy Rust Template
Repository of Raspberry Pi Pico with Rust 🦀 + Embassy Framework

## Prerequisites:

#### Adding Target thumbv6m-none-eabi
Move to the Project Directory and Run this following command:
```bash
rustup target add thumbv6m-none-eabi
```

#### Installing elf2uf2-rs package
##### Dependency
```bash
sudo apt install -y pkg-config libusb-1.0-0-dev libftdi1-dev
sudo apt-get install libudev-dev
```
##### Installation
```bash
cargo install elf2uf2-rs
```



