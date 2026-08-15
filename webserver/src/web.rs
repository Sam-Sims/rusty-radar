use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::{Aircraft, DIST, PositionOffset, RecieverLocation};

#[derive(Debug, Deserialize)]
struct Response {
    ac: Vec<RawAPIAircraft>,
    now: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawAPIAircraft {
    hex: Option<String>,
    #[serde(rename = "flight")]
    callsign: Option<String>,
    #[serde(rename = "t")]
    aircraft_type: Option<String>,
    #[serde(rename = "desc")]
    description: Option<String>,
    // api is pesky and alt_baro can report a string - "ground" and u32.
    alt_baro: Option<serde_json::Value>,
    lat: Option<f64>,
    lon: Option<f64>,
    track: Option<f32>,
    squawk: Option<String>,
}

impl TryFrom<(RawAPIAircraft, u64, &RecieverLocation)> for Aircraft {
    type Error = anyhow::Error;

    fn try_from(
        (
            raw,
            timestamp,
            RecieverLocation {
                lat: recv_lat,
                lon: recv_lon,
            },
        ): (RawAPIAircraft, u64, &RecieverLocation),
    ) -> Result<Self, Self::Error> {
        let clean = |value: Option<String>| {
            value
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .unwrap_or("unkwn")
                .to_owned()
        };
        let hex = raw.hex.ok_or_else(|| anyhow!("aircraft has no hex"))?;
        let hex = hex.trim().to_owned();
        let callsign = clean(raw.callsign);
        let aircraft_type = clean(raw.aircraft_type);
        let description = clean(raw.description);
        let altitude = raw
            .alt_baro
            .and_then(|value| value.as_u64())
            .and_then(|value| u32::try_from(value).ok())
            .unwrap_or(0);
        let flight_level = format!("FL{:03}", (f64::from(altitude) / 100.0).round() as u32,);
        let squawk = raw
            .squawk
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());
        let lat = raw.lat.ok_or_else(|| anyhow!("aircraft has no lat"))?;
        let lon = raw.lon.ok_or_else(|| anyhow!("aircraft has no lon"))?;
        let track = raw.track;
        let label = format!("{callsign}\n{flight_level}\n{aircraft_type}");

        let position_offset =
            PositionOffset::new(recv_lat.to_owned(), recv_lon.to_owned(), lat, lon);

        Ok(Self {
            hex,
            callsign,
            aircraft_type,
            description,
            altitude,
            flight_level,
            squawk,
            lat,
            lon,
            track,
            label,
            timestamp,
            position_offset,
        })
    }
}

pub async fn poll_web_aicraft(
    client: &reqwest::Client,
    my_location: &RecieverLocation,
) -> Result<Vec<Aircraft>> {
    let RecieverLocation { lat, lon } = my_location;
    let url = format!("https://opendata.adsb.fi/api/v3/lat/{lat}/lon/{lon}/dist/{DIST}");
    let resp = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json::<Response>()
        .await?;

    // println!("{}", serde_json::to_string_pretty(&resp).unwrap());
    let timestamp = resp.now;

    let aircraft = resp
        .ac
        .into_iter()
        .filter_map(|raw| Aircraft::try_from((raw, timestamp, my_location)).ok())
        .collect();

    Ok(aircraft)
}
