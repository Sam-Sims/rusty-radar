use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, ascii::*},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use micromath::F32Ext;

use crate::RadarScale;

const LABEL_FONT: &MonoFont<'static> = &FONT_5X8;
const AIRCRAFT_COLOUR: Rgb565 = Rgb565::GREEN;
const LABEL_COLOUR: Rgb565 = Rgb565::CSS_LIGHT_GRAY;
const TRACK_LENGTH: f32 = 10.0;

pub struct Aircraft<'a> {
    x: f32,
    y: f32,
    heading: f32,
    label: &'a str,
}

impl<'a> Aircraft<'a> {
    pub fn new(x: f32, y: f32, heading: f32, label: &'a str) -> Self {
        Self {
            x,
            y,
            heading,
            label,
        }
    }

    pub(crate) fn draw<D>(
        &self,
        target: &mut D,
        top_heading: f32,
        center: Point,
        radar_radius: i32,
        scale: RadarScale,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let position = self.position_to_radar(center, radar_radius, scale, top_heading);
        self.draw_track(target, position)?;
        self.draw_symbol(target, position)?;
        self.draw_label(target, position)?;

        Ok(())
    }

    fn draw_track<D>(&self, target: &mut D, position: Point) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let direction = caclulate_heading_point(self.heading);

        Line::new(position, position + direction)
            .into_styled(PrimitiveStyle::with_stroke(AIRCRAFT_COLOUR, 1))
            .draw(target)?;
        Ok(())
    }

    fn draw_symbol<D>(&self, target: &mut D, position: Point) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(AIRCRAFT_COLOUR)
            .stroke_width(1)
            .build();

        Rectangle::new(position - Point::new(2, 2), Size::new(5, 5))
            .into_styled(style)
            .draw(target)?;

        Ok(())
    }

    fn draw_label<D>(&self, target: &mut D, position: Point) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        // for now labels always at 90deg offset
        let track = caclulate_heading_point(self.heading);
        let label_start_point = Point::new(-track.y, track.x);
        let label_end_point = position + label_start_point;

        let alignment = if label_start_point.x > 0 {
            Alignment::Left
        } else if label_start_point.x < 0 {
            Alignment::Right
        } else {
            Alignment::Center
        };

        let label = MonoTextStyle::new(LABEL_FONT, AIRCRAFT_COLOUR);
        let label_text_style = TextStyleBuilder::new()
            .baseline(Baseline::Top)
            .alignment(alignment)
            .build();

        Line::new(position, label_end_point)
            .into_styled(PrimitiveStyle::with_stroke(LABEL_COLOUR, 1))
            .draw(target)?;

        let mut label_text =
            Text::with_text_style(self.label, label_end_point, label, label_text_style);

        let label_bounds = label_text.bounding_box();
        let label_height = label_bounds.size.height as i32;

        let label_y = if label_start_point.y > 0 {
            label_end_point.y
        } else if label_start_point.y < 0 {
            label_end_point.y - label_height
        } else {
            label_end_point.y - label_height / 2
        };

        label_text.translate_mut(Point::new(0, label_y - label_bounds.top_left.y));
        label_text.draw(target)?;

        Ok(())
    }

    fn position_to_radar(
        &self,
        center: Point,
        radar_radius: i32,
        scale: RadarScale,
        top_heading: f32,
    ) -> Point {
        let pixels_per_meter = radar_radius as f32 / scale.to_meters() as f32;

        let (sin, cos) = top_heading.to_radians().sin_cos();

        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;

        center
            + Point::new(
                (x * pixels_per_meter).round() as i32,
                (y * pixels_per_meter).round() as i32,
            )
    }
}

fn caclulate_heading_point(heading: f32) -> Point {
    // let vector_bin = (heading as usize * HEADING_VECTOR.len()) / 360;
    // let (x, y) = HEADING_VECTOR[vector_bin];
    // Point::new(x, y)
    let (sin, cos) = heading.to_radians().sin_cos();
    Point::new(
        (sin * TRACK_LENGTH).round() as i32,
        (-cos * TRACK_LENGTH).round() as i32,
    )
}
