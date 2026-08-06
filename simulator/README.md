# simulator

Simulation 240x240 display for local testing without needing/flashing to a physical device.

## Usage

```bash
cargo run -p rusty-radar-simulator
```

For local dev having cargo-watch rebuild and run on changes is useful:

```bash
cargo watch -w simulator/src -w simulator/Cargo.toml -w graphics/src -w graphics/Cargo.toml -x 'run --target x86_64-unknown-linux-gnu -p rusty-radar-simulator'
```

Once happy with implementation - flash

```bash
cargo espflash flash --monitor
```
