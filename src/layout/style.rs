use crate::graphics::color::Color;

#[derive(Clone, Copy)]
pub struct Style {
    pub size: f32,
    pub bold: bool,
    pub italic: bool,
    pub color: Color,
}

impl Style {
    pub fn new(size: f32, bold: bool, italic: bool, color: Color) -> Self {
        Self {
            size,
            bold,
            italic,
            color,
        }
    }
}

pub fn block_style(tag: &str) -> Style {
    let black = Color::new(0.0, 0.0, 0.0);

    match tag {
        "h1" => Style::new(32.0, true, false, black),
        "h2" => Style::new(28.0, true, false, black),
        "h3" => Style::new(24.0, true, false, black),
        "h4" => Style::new(20.0, true, false, black),
        "h5" => Style::new(18.0, true, false, black),
        "h6" => Style::new(16.0, true, false, black),
        _ => Style::new(16.0, false, false, black),
    }
}

pub fn inline_style(tag: &str, base: Style) -> Style {
    match tag {
        "b" | "strong" => Style { bold: true, ..base },
        "i" | "em" => Style {
            italic: true,
            ..base
        },
        "a" => Style {
            color: Color::new(0.1, 0.3, 0.9),
            ..base
        },
        _ => base,
    }
}
