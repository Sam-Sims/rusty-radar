#![no_std]
pub mod aircraft;
mod radar;

use embedded_graphics::{
    mono_font::{
        MonoFont, MonoTextStyle,
        ascii::{FONT_6X9, FONT_9X18_BOLD},
    },
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};

pub use crate::aircraft::Aircraft;

// config
const COMPASS_HEADING_FONT: &MonoFont<'static> = &FONT_9X18_BOLD;
const SCALE_FONT: &MonoFont<'static> = &FONT_6X9;
const RADAR_COLOUR: Rgb565 = Rgb565::WHITE;

// dont actually need this in the end - so set at 0
// but a useful opt to have
const OUTER_MARGIN: i32 = 0;
const SCALE_X_OFFSET: i32 = 15;
const SCALE_Y_OFFSET: i32 = 7;

#[derive(Clone, Copy)]
pub enum RadarScale {
    Km5,
    Km10,
    Km30,
}

impl RadarScale {
    pub fn next(self) -> Self {
        match self {
            Self::Km5 => Self::Km10,
            Self::Km10 => Self::Km30,
            Self::Km30 => Self::Km5,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Km5 => "5km",
            Self::Km10 => "10km",
            Self::Km30 => "20km",
        }
    }

    fn to_meters(self) -> u32 {
        match self {
            Self::Km5 => 5000,
            Self::Km10 => 10000,
            Self::Km30 => 30000,
        }
    }
}

pub fn draw<D>(
    target: &mut D,
    scale: RadarScale,
    aircraft: &[Aircraft],
    top_heading: f32,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let bounds = target.bounding_box();
    let center = bounds.center();
    let radar_radius = radar::calculate_radius(bounds);

    target.clear(Rgb565::BLACK)?;
    radar::draw_face(target, top_heading, center, radar_radius)?;
    draw_scale(target, scale)?;
    draw_planes(target, aircraft, top_heading, center, radar_radius, scale)?;
    Ok(())
}

fn draw_planes<D>(
    target: &mut D,
    planes: &[Aircraft],
    top_heading: f32,
    center: Point,
    radar_radius: i32,
    scale: RadarScale,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    for plane in planes {
        plane.draw(target, top_heading, center, radar_radius, scale)?;
    }

    Ok(())
}

fn draw_scale<D>(target: &mut D, scale: RadarScale) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let bounds = target.bounding_box();
    let center = bounds.center();

    let right = bounds.top_left.x + bounds.size.width as i32 - 1;

    let text_width = COMPASS_HEADING_FONT.character_size.width as i32;

    let pos_x = right - text_width - SCALE_X_OFFSET;
    let pos_y = center.y + SCALE_Y_OFFSET;

    let scale_style = MonoTextStyle::new(SCALE_FONT, Rgb565::WHITE);
    let scale_text_style = TextStyleBuilder::new()
        .alignment(Alignment::Right)
        .baseline(Baseline::Middle)
        .build();

    Text::with_text_style(
        scale.label(),
        Point::new(pos_x, pos_y),
        scale_style,
        scale_text_style,
    )
    .draw(target)?;
    Ok(())
}
