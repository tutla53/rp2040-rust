# Raspberry Pi Pico RP2040 + Embassy Rust Template
***

### Module
- [Rust](https://www.rust-lang.org/tools/install)🦀
- [Embassy-Rust](https://github.com/embassy-rs/embassy)
- [probe-rs](https://probe.rs/) &#8594; If you use Debug Probe
- [elf2uf2](https://crates.io/crates/elf2uf2-rs/versions)
***

## Getting Started:
### Cloning the Repository
- `git clone` this repository with this command:
    ```bash
    git clone --recurse-submodules https://github.com/tutla53/rp2040.git 
    ```
- If you didn't use a `--recursive` git clone, then you need to make
  sure that `embassy-rs` is fetched now. From the top level apply
  one of:
  ```bash
  git submodule update --init --recursive   # First time
  git submodule update --recursive          # Subsequent
  ```
  
### Installing the Module  
Move to the Project Directory e.g. `0_template`:
```bash
cd rp2040-rust/0_template/
```

#### Adding the Build Target
Run this following command to add the build target:
```bash
rustup target add thumbv6m-none-eabi
```
  
#### Adding probe-rs Package
##### Installation
  ```bash
  curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
  ```
| :warning: WARNING          |
|:---------------------------|
|Don't run `cargo add probe-rs`|
#### Installing elf2uf2-rs Package

##### Dependency
  ```bash
  sudo apt install -y pkg-config libusb-1.0-0-dev libftdi1-dev && sudo apt-get install libudev-dev
  ```
##### Installation
  ```bash
  cargo install elf2uf2-rs
  ```
### Build and Run the First Program
#### Build
  ```bash
  cargo build --release
  ```

#### Run 
Please select one of the runner on the `.cargo/config.toml`
```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "probe-rs run --chip RP2040"  # Directly Flash with Debug Probe
runner = "elf2uf2-rs -d"               # Automatic deployment to a mounted Pico
runner = "elf2uf2-rs"                  # Only create uf2 file and can be flashed to pico by pressing the bootsel button
```
The run the following code:
```bash
cargo run --release
```

The compiled `elf` and `uf2` file will be in the `target/thumbv6m-none-eabi/release/rp2040-template`



