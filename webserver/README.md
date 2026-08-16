# webserver

Axum webserver exposing an `/api/aircraft` JSON endpoint that returns

```rust
#[derive(Debug, Clone, Serialize)]
pub struct Aircraft {
    hex: String,
    callsign: String,
    aircraft_type: String,
    description: String,
    altitude: u32,
    flight_level: String,
    squawk: Option<String>,
    lat: f64,
    lon: f64,
    track: Option<f32>,
    label: String,
    timestamp: u64,
    position_offset: PositionOffset,
}
```

Will expose data from adsbi.fi OR an RTL-SDR with dump1090 (toggle with a switch). Both sources are normalised to `Aircraft`

## Usage

```bash
just webserver run
```

Webserver runs on port 3000.

To read JSON from the endpoint, construct a HTTP `get` and deserialize with `serde` taking only the fields needed  e.g

```rust
#[derive(Deserialize, Debug)]
struct ApiAircraft {
    position_offset: PositionOffset,
    track: Option<f64>,
    label: String,
}
```
