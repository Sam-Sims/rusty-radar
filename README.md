# rusty radar

A Rust implementation of an ESP32-S3/GC9A01 based aircraft tracker using live data from an ADS-B antenna.

This repo documents notes/progress towards a working aircraft "radar" using data from an ADS-B antenna + SDR.

## Components:
- ESP32-S3 board
- GC9A01 1.28" LCD
- Random coax cable [for an antenna](https://discussions.flightaware.com/t/quick-spider-no-soldering-no-connector/19181)
- 1090mhz bandpass filter + LNA [cheap combo thing](https://www.aliexpress.com/item/1005004987445918.html)
- RTL SDR dongle

## Setup

The ESP32-S3 is based on the Xtensa architecture unlike the ESP32-C (which is RISC-V) so it is not as straightforward to get started. Xtensa is not yet officially supported by Rust as Rust uses LLVM, and [LLVM does not yet support Xtensa](https://github.com/espressif/llvm-project/issues/4). However espressif mantain custom forks of both LLVM and the Rust compiler that include Xtensa support which can easily be installed via the [espup](https://github.com/esp-rs/espup) tool.

Setup is as simple as:

```bash
cargo install espup --locked
espup install --targets esp32s3
```

You will need to set the `LIBCLANG_PATH` as well as adding `xtensa-esp-elf/bin` to `PATH`. This step must be done every time you open a new terminal. espup creates a script for you to this, or stick them in your shell of choices RC file.

### Generating a project

We can use `cargo generate` to setup a template for our project:

```bash
cargo generate esp-rs/esp-idf-template
cargo build
```

### Flashing

Install espflash:

```bash
cargo install espflash --locked
```

Check that your device can be seen:

```bash
cargo espflash list-ports --list-all-ports
```

Initially I got an error that no serial devices could be found - before realisng my user wasnt in the `uugc` group:

```bash
sudo usermod -aG uucp "$USER"
```

and a reboot soon fixed that


Then we can do

```bash
cargo espflash flash --monitor
```

and you should ideally see something like:

```
rusty-radar main ? ❯ cargo espflash flash --monitor
[2026-08-03T14:28:23Z INFO ] Serial port: '/dev/ttyACM0'
[2026-08-03T14:28:23Z INFO ] Connecting...
[2026-08-03T14:28:24Z INFO ] Using flash stub
   Compiling bindgen v0.71.1
   Compiling embuild v0.33.3
   Compiling esp-idf-sys v0.37.2 (https://github.com/esp-rs/esp-idf-sys.git#8369f610)
   Compiling esp-idf-hal v0.46.2 (https://github.com/esp-rs/esp-idf-hal.git#f1bac2d9)
   Compiling esp-idf-svc v0.52.1 (https://github.com/esp-rs/esp-idf-svc.git#f59ffada)
   Compiling rusty-radar v0.1.0 (/mnt/SSD2/arch/projects/rusty-radar)
    Finished `dev` profile [optimized + debuginfo] target(s) in 38.68s
Chip type:         esp32s3 (revision v0.2)
Crystal frequency: 40 MHz
Flash size:        16MB
Features:          WiFi, BLE, Embedded Flash
MAC address:       8c:fd:49:9b:f2:7c
App/part. size:    584,624/1,048,576 bytes, 55.75%
[00:00:01] [========================================]       1/1       0x0      Verifying... OK!  [00:00:00] [========================================]       1/1       0x8000   Verifying... OK!  [00:00:26] [========================================]      18/18      0x10000  Verifying... OK!  [2026-08-03T14:29:32Z INFO ] Flashing has completed!
```

## Drivers

For the display we need a driver:

```bash
cargo add gc9a01-rs
```

For the touchscreen, the driver by the same author isnt published as a crate, but we can add it manually through the git repo into our `Cargo.toml`

```
cst816s-rs = { git = "https://github.com/IniterWorker/cst816s", rev = "966a0761f992c63ea1a953ba3d4a3fab8de15db0", features = [
    "loglib",
    "std",
] }
```

There is a nice example of how to use these [here](https://github.com/IniterWorker/esp32-s3-touch-lcd-1-28)

## Development

The repo is split into a couple crates:

[firmware](firmware/README.md)
[graphics](graphics/README.md)
[simulator](simulator/README.md)

See each link for more information.

## References
### GPIO pinout
![pinout](https://docs.waveshare.com/assets/images/ESP32-S3-Touch-LCD-1.28-pin-4c11432ab715535bf20f334d46ebefbf.webp)
