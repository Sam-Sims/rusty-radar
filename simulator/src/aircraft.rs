use rand::{rngs::SmallRng, Rng, SeedableRng};
use rusty_radar_graphics::Aircraft;

const SPEED: f32 = 5.0;

pub struct SimCraft {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    heading: u16,
    label: String,
}

impl SimCraft {
    pub fn new() -> Self {
        let mut rng = SmallRng::from_os_rng();

        let heading = rng.random_range(0..360);
        let heading_rad = (heading as f32).to_radians();

        let callsign = format!(
            "{}{}{:03}",
            rng.random_range(b'A'..=b'Z') as char,
            rng.random_range(b'A'..=b'Z') as char,
            rng.random_range(0..999),
        );
        let flight_level = rng.random_range(10..=400);

        Self {
            x: rng.random_range(30.0..210.0),
            y: rng.random_range(30.0..210.0),
            dx: heading_rad.sin() * SPEED,
            dy: -heading_rad.cos() * SPEED,
            heading,
            label: format!("{callsign}\nFL{flight_level}"),
        }
    }

    pub fn update(&mut self) -> bool {
        // returns false if out of range of display
        self.x += self.dx;
        self.y += self.dy;

        (0.0..240.0).contains(&self.x) && (0.0..240.0).contains(&self.y)
    }

    pub fn to_aircraft(&self) -> Aircraft<'_> {
        Aircraft::new(self.x as i32, self.y as i32, self.heading, &self.label)
    }
}
