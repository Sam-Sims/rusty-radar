use embedded_graphics::{
    mono_font::{
        MonoFont, MonoTextStyle,
        ascii::{FONT_6X9, FONT_9X18_BOLD},
    },
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use micromath::F32Ext;

// config
const COMPASS_HEADING_FONT: &MonoFont<'static> = &FONT_9X18_BOLD;
const SCALE_FONT: &MonoFont<'static> = &FONT_6X9;
const RADAR_COLOUR: Rgb565 = Rgb565::WHITE;

// dont actually need this in the end - so set at 0
// but a useful opt to have
const OUTER_MARGIN: i32 = 0;
const SCALE_X_OFFSET: i32 = 15;
const SCALE_Y_OFFSET: i32 = 7;

pub(crate) fn draw_face<D>(
    target: &mut D,
    top_heading: f32,
    center: Point,
    radar_radius: i32,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let compass_radius = radar_radius + COMPASS_HEADING_FONT.character_size.height as i32 / 2;
    let radar_diameter = (radar_radius * 2) as u32;
    draw_radar_rings(target, center, radar_diameter)?;
    draw_radar_crosshairs(target, center, radar_radius, top_heading)?;
    draw_radar_compass(target, center, top_heading, compass_radius)?;

    Ok(())
}

pub fn calculate_radius(bounds: Rectangle) -> i32 {
    let text_size = COMPASS_HEADING_FONT.character_size;
    let text_width = text_size.width as i32;
    let text_height = text_size.height as i32;

    let availible_width = bounds.size.width as i32 - 2 * (text_width + OUTER_MARGIN);
    let availible_height = bounds.size.height as i32 - 2 * (text_height + OUTER_MARGIN);

    let outer_diameter = availible_width.min(availible_height) as u32;
    outer_diameter as i32 / 2
}

fn draw_radar_rings<D>(target: &mut D, center: Point, outer_diameter: u32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
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
    Ok(())
}

fn draw_radar_crosshairs<D>(
    target: &mut D,
    center: Point,
    outer_radius: i32,
    top_heading: f32,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let north = point_at_heading(center, relative_heading(0.0, top_heading), outer_radius);
    let east = point_at_heading(center, relative_heading(90.0, top_heading), outer_radius);
    let south = point_at_heading(center, relative_heading(180.0, top_heading), outer_radius);
    let west = point_at_heading(center, relative_heading(270.0, top_heading), outer_radius);

    Line::new(north, south)
        .into_styled(PrimitiveStyle::with_stroke(RADAR_COLOUR, 2))
        .draw(target)?;

    Line::new(east, west)
        .into_styled(PrimitiveStyle::with_stroke(RADAR_COLOUR, 2))
        .draw(target)?;
    Ok(())
}

fn draw_radar_compass<D>(
    target: &mut D,
    center: Point,
    top_heading: f32,
    compass_radius: i32,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let compass_style = MonoTextStyle::new(COMPASS_HEADING_FONT, Rgb565::RED);
    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Middle)
        .build();

    for (heading_label, true_heading) in [("N", 0.0), ("E", 90.0), ("S", 180.0), ("W", 270.0)] {
        let heading = relative_heading(true_heading, top_heading);
        let position = point_at_heading(center, heading, compass_radius);

        Text::with_text_style(heading_label, position, compass_style, text_style).draw(target)?;
    }
    Ok(())
}

fn relative_heading(true_heading: f32, top_heading: f32) -> f32 {
    (true_heading - top_heading).rem_euclid(360.0)
}

fn point_at_heading(center: Point, heading: f32, distance: i32) -> Point {
    let (sin, cos) = heading.to_radians().sin_cos();

    center
        + Point::new(
            (sin * distance as f32).round() as i32,
            (-cos * distance as f32).round() as i32,
        )
}
