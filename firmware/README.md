# firmware

Crate containing firmware+drivers for ESP32-S3 + display.

## Usage

To flash

```bash
export LIBCLANG_PATH="/mnt/SSD2/arch/tools/rustup/toolchains/esp/xtensa-esp32-elf-clang/esp-20.1.1_20250829/esp-clang/lib"
export PATH="/mnt/SSD2/arch/tools/rustup/toolchains/esp/xtensa-esp-elf/esp-15.2.0_20250920/xtensa-esp-elf/bin:$PATH"
export RUSTC_WRAPPER=sccache
export CC="sccache gcc"

cargo espflash flash --monitor
```
