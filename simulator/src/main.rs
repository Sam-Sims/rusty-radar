use std::{thread, time::Duration, time::Instant};

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use rusty_radar_graphics::{RadarScale, aircraft::Aircraft, draw_frame};

use crate::aircraft::SimCraft;

pub mod aircraft;


fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(240, 240));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Clock", &output_settings);

    // mock planes
    let mut planes = [
        SimCraft::new(),
        SimCraft::new(),
        SimCraft::new(),
        SimCraft::new(),
        SimCraft::new(),
    ];

    let update_interval = Duration::from_secs(2);
    let mut next_update = Instant::now();

    'running: loop {
        if Instant::now() >= next_update {
            for plane in &mut planes {
                if !plane.update() {
                    *plane = SimCraft::new();
                }
            }
            let aircraft = planes.each_ref().map(SimCraft::to_aircraft);
            // display.clear(Rgb565::BLACK)?;
            // rusty_radar_graphics::draw_face(&mut display)?;
            // rusty_radar_graphics::draw_scale(&mut display, rusty_radar_graphics::RadarScale::Km5)?;
            rusty_radar_graphics::draw_frame(&mut display, RadarScale::Km5)?;
            rusty_radar_graphics::draw_planes(&mut display, &aircraft)?;
            window.update(&display);
            next_update += update_interval;
        }
        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }

        thread::sleep(Duration::from_millis(50));
    }
}
