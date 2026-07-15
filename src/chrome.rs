use crate::graphics::color::Color;
use crate::graphics::display_list::DisplayCommand;

pub const HEIGHT: f32 = 56.0;

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    fn centered_baseline(&self, size: f32) -> f32 {
        self.y + self.height / 2.0 + size * 0.35
    }
}

pub struct Chrome {
    width: f32,
    pub home_button: Rect,
    pub reload_button: Rect,
    pub field: Rect,
}

impl Chrome {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            home_button: Rect {
                x: 12.0,
                y: 10.0,
                width: 70.0,
                height: 36.0,
            },
            reload_button: Rect {
                x: 92.0,
                y: 10.0,
                width: 80.0,
                height: 36.0,
            },
            field: Rect {
                x: 182.0,
                y: 10.0,
                width: width - 182.0 - 12.0,
                height: 36.0,
            },
        }
    }

    pub fn commands(&self, address: &str) -> Vec<DisplayCommand> {
        let mut commands = Vec::new();

        commands.push(DisplayCommand::Rect {
            x: 0.0,
            y: 0.0,
            width: self.width,
            height: HEIGHT,
            color: Color::new(0.12, 0.13, 0.18),
        });

        self.push_button(&mut commands, &self.home_button, "Home");
        self.push_button(&mut commands, &self.reload_button, "Reload");

        commands.push(DisplayCommand::Rect {
            x: self.field.x - 2.0,
            y: self.field.y - 2.0,
            width: self.field.width + 4.0,
            height: self.field.height + 4.0,
            color: Color::new(0.3, 0.5, 0.9),
        });

        commands.push(DisplayCommand::Rect {
            x: self.field.x,
            y: self.field.y,
            width: self.field.width,
            height: self.field.height,
            color: Color::new(1.0, 1.0, 1.0),
        });

        commands.push(DisplayCommand::Text {
            text: format!("{address}|"),
            x: self.field.x + 8.0,
            y: self.field.centered_baseline(18.0),
            size: 18.0,
            bold: false,
            italic: false,
            color: Color::new(0.0, 0.0, 0.0),
        });

        commands
    }

    fn push_button(&self, commands: &mut Vec<DisplayCommand>, rect: &Rect, label: &str) {
        commands.push(DisplayCommand::Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
            color: Color::new(0.85, 0.87, 0.92),
        });

        commands.push(DisplayCommand::Text {
            text: label.to_string(),
            x: rect.x + 10.0,
            y: rect.centered_baseline(14.0),
            size: 14.0,
            bold: true,
            italic: false,
            color: Color::new(0.1, 0.1, 0.15),
        });
    }
}
