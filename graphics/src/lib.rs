#![no_std]
pub mod aircraft;

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
}

pub fn draw_frame<D>(target: &mut D, scale: RadarScale) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    target.clear(Rgb565::BLACK)?;
    draw_face(target)?;
    draw_scale(target, scale)?;

    Ok(())
}

pub fn draw_planes<D>(target: &mut D, planes: &[Aircraft<'_>]) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    for plane in planes {
        plane.draw(target)?;
    }

    Ok(())
}

pub fn draw_scale<D>(target: &mut D, scale: RadarScale) -> Result<(), D::Error>
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

pub fn draw_face<D>(target: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let bounds = target.bounding_box();
    let center = bounds.center();

    let left = bounds.top_left.x;
    let top = bounds.top_left.y;
    let right = left + bounds.size.width as i32 - 1;
    let bottom = top + bounds.size.height as i32 - 1;

    let compass_style = MonoTextStyle::new(COMPASS_HEADING_FONT, Rgb565::RED);
    let text_size = COMPASS_HEADING_FONT.character_size;
    let text_width = text_size.width as i32;
    let text_height = text_size.height as i32;

    let availible_width = bounds.size.width as i32 - 2 * (text_width + OUTER_MARGIN);
    let availible_height = bounds.size.height as i32 - 2 * (text_height + OUTER_MARGIN);

    let outer_diameter = availible_width.min(availible_height) as u32;
    let outer_radius = outer_diameter as i32 / 2;

    let north_text_style = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Top)
        .build();

    let south_text_style = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Bottom)
        .build();

    let east_text_style = TextStyleBuilder::new()
        .alignment(Alignment::Right)
        .baseline(Baseline::Middle)
        .build();

    let west_text_style = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Middle)
        .build();

    Text::with_text_style(
        "N",
        Point::new(center.x, top),
        compass_style,
        north_text_style,
    )
    .draw(target)?;

    Text::with_text_style(
        "S",
        Point::new(center.x, bottom),
        compass_style,
        south_text_style,
    )
    .draw(target)?;

    Text::with_text_style(
        "E",
        Point::new(right, center.y),
        compass_style,
        east_text_style,
    )
    .draw(target)?;

    Text::with_text_style(
        "W",
        Point::new(left, center.y),
        compass_style,
        west_text_style,
    )
    .draw(target)?;

    for d in [
        outer_diameter,
        outer_diameter * 130 / 190,
        outer_diameter * 70 / 190,
        outer_diameter * 30 / 190,
    ] {
        Circle::with_center(center, d)
            .into_styled(PrimitiveStyle::with_stroke(RADAR_COLOUR, 2))
            .draw(target)?;
    }

    Line::new(
        Point::new(center.x - outer_radius, center.y),
        Point::new(center.x + outer_radius, center.y),
    )
    .into_styled(PrimitiveStyle::with_stroke(RADAR_COLOUR, 2))
    .draw(target)?;

    Line::new(
        Point::new(center.x, center.y - outer_radius),
        Point::new(center.x, center.y + outer_radius),
    )
    .into_styled(PrimitiveStyle::with_stroke(RADAR_COLOUR, 2))
    .draw(target)?;

    Ok(())
}
