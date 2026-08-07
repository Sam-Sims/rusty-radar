# firmware

Firmware and drivers an for ESP32-S3 board with a 1.28" GC9A01 display and CST816S touch controller.

### Drivers
- Display drivers: [gc9a01](https://docs.rs/gc9a01-rs/0.4.2/gc9a01/index.html)
- Touch drivers: [cst816s](https://github.com/IniterWorker/cst816s)

### Rendering

Graphics are handeled in the [graphics](../graphics/README.md) crate.

## Usage

Easiest way to flash is to use `just` from the repo root. This makes sure the right toolchain and cargo settings are used

```bash
just firmware flash
```

if you want to monitor serial output

```bash
just firmware flashm
```

Or flash manually - make sure you `cd` into firmware or cargo wont respect the `.cargo/config.toml`
```bash
cd firmware
export LIBCLANG_PATH="/mnt/SSD2/arch/tools/rustup/toolchains/esp/xtensa-esp32-elf-clang/esp-20.1.1_20250829/esp-clang/lib"
export PATH="/mnt/SSD2/arch/tools/rustup/toolchains/esp/xtensa-esp-elf/esp-15.2.0_20250920/xtensa-esp-elf/bin:$PATH"
export RUSTC_WRAPPER=sccache
export CC="sccache gcc"

cargo espflash flash --monitor
```
