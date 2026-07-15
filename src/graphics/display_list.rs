use super::color::Color;

pub enum DisplayCommand {
    Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
    },

    Text {
        text: String,
        x: f32,
        y: f32,
        size: f32,
        bold: bool,
        italic: bool,
        color: Color,
    },
}
