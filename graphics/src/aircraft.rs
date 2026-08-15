use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, ascii::*},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};

const LABEL_FONT: &MonoFont<'static> = &FONT_5X8;
const AIRCRAFT_COLOUR: Rgb565 = Rgb565::GREEN;
const LABEL_COLOUR: Rgb565 = Rgb565::CSS_LIGHT_GRAY;

// this is a no trigonometry zone
// pre-computed vectors for 16 directions which should be enough resolution on a small display
const HEADING_VECTOR: [(i32, i32); 16] = [
    (0, -10),
    (4, -9),
    (7, -7),
    (9, -4),
    (10, 0),
    (9, 4),
    (7, 7),
    (4, 9),
    (0, 10),
    (-4, 9),
    (-7, 7),
    (-9, 4),
    (-10, 0),
    (-9, -4),
    (-7, -7),
    (-4, -9),
];

pub struct Aircraft<'a> {
    pos: Point,
    heading: u16,
    label: &'a str,
}

impl<'a> Aircraft<'a> {
    pub fn new(x: i32, y: i32, heading: u16, label: &'a str) -> Self {
        Self {
            pos: Point::new(x, y),
            heading,
            label,
        }
    }

    pub fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        self.draw_track(target)?;
        self.draw_symbol(target)?;
        self.draw_label(target)?;

        Ok(())
    }

    fn draw_track<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let direction = caclulate_heading_point(self.heading);

        Line::new(self.pos, self.pos + direction)
            .into_styled(PrimitiveStyle::with_stroke(AIRCRAFT_COLOUR, 1))
            .draw(target)?;
        Ok(())
    }

    fn draw_symbol<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(AIRCRAFT_COLOUR)
            .stroke_width(1)
            .build();

        Rectangle::new(self.pos - Point::new(2, 2), Size::new(5, 5))
            .into_styled(style)
            .draw(target)?;

        Ok(())
    }

    fn draw_label<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        // for now labels always at 90deg offset
        let track = caclulate_heading_point(self.heading);
        let label_start_point = Point::new(-track.y, track.x);
        let label_end_point = self.pos + label_start_point;

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

        Line::new(self.pos, label_end_point)
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
}

fn caclulate_heading_point(heading: u16) -> Point {
    let vector_bin = (heading as usize * HEADING_VECTOR.len()) / 360;
    let (x, y) = HEADING_VECTOR[vector_bin];
    Point::new(x, y)
}
