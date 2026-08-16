use std::{
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use rusty_radar_graphics::RadarScale;
use serde::Deserialize;

use crate::simcraft::SimCraft;

pub mod simcraft;

#[derive(Deserialize)]
struct PositionOffset {
    x: f64,
    y: f64,
}

#[derive(Deserialize)]
struct RawAPIArcraft {
    position_offset: PositionOffset,
    track: Option<f64>,
    label: String,
}

fn fetch_aircraft(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<Vec<RawAPIArcraft>, reqwest::Error> {
    let resp = client.get(url).send()?.error_for_status()?;
    resp.json::<Vec<RawAPIArcraft>>()
}

fn track_to_heading(track: f64) -> Option<u16> {
    if !track.is_finite() {
        return None;
    }
    let deg = track.rem_euclid(360.0).round() as u16;
    Some(deg % 360)
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(240, 240));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Clock", &output_settings);
    let client = reqwest::blocking::Client::new();
    let url = "http://127.0.0.1:3000/api/aircraft";

    // // mock planes
    // let mut planes = [
    //     SimCraft::new(),
    //     SimCraft::new(),
    //     SimCraft::new(),
    //     SimCraft::new(),
    //     SimCraft::new(),
    // ];
    //

    let update_interval = Duration::from_secs(2);
    let mut next_update = Instant::now();

    let mut raw_aircraft: Vec<RawAPIArcraft> = Vec::new();

    'running: loop {
        if Instant::now() >= next_update {
            match fetch_aircraft(&client, url) {
                Ok(new_aircraft) => {
                    raw_aircraft = new_aircraft;
                }
                Err(error) => {
                    panic!()
                }
            }

            let aircraft = raw_aircraft
                .iter()
                .map(|aircraft| {
                    let track = aircraft.track.and_then(track_to_heading).unwrap_or(0);
                    rusty_radar_graphics::Aircraft::new(
                        aircraft.position_offset.x as f32,
                        aircraft.position_offset.y as f32,
                        track,
                        &aircraft.label,
                    )
                })
                .collect::<Vec<_>>();

            // for plane in &mut planes {
            //     if !plane.update() {
            //         *plane = SimCraft::new();
            //     }
            // }
            // let aircraft = planes.each_ref().map(SimCraft::to_aircraft);
            // display.clear(Rgb565::BLACK)?;
            // rusty_radar_graphics::draw_face(&mut display)?;
            // rusty_radar_graphics::draw_scale(&mut display, rusty_radar_graphics::RadarScale::Km5)?;
            // rusty_radar_graphics::draw_frame(&mut display, RadarScale::Km5)?;
            // rusty_radar_graphics::draw_planes(&mut display, &aircraft)?;
            rusty_radar_graphics::draw(&mut display, RadarScale::Km30, &aircraft, 156.0);
            window.update(&display);
            next_update += update_interval;
        }
        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }

        thread::sleep(Duration::from_millis(50));
    }
}
