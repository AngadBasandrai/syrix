use crate::dom::structures::Node;
use crate::graphics::text::FontSet;

use super::display::{Display, default_display};
use super::style::{Style, inline_style};

pub struct TextRun {
    text: String,
    x: f32,
    y: f32,
    style: Style,
}

impl TextRun {
    pub fn new(text: String, x: f32, y: f32, style: Style) -> Self {
        Self { text, x, y, style }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn style(&self) -> Style {
        self.style
    }
}

pub fn is_inline_node(node: &Node) -> bool {
    match node {
        Node::Text(_) => true,
        Node::Element(element) => matches!(default_display(element.tag()), Display::Inline),
    }
}

fn flatten(node: &Node, style: Style, out: &mut Vec<(char, Style)>) {
    match node {
        Node::Text(text) => {
            for c in text.content().chars() {
                out.push((c, style));
            }
        }
        Node::Element(element) => {
            let style = inline_style(element.tag(), style);
            for child in element.children() {
                flatten(child, style, out);
            }
        }
    }
}

pub fn layout_inline(
    nodes: &[&Node],
    x: f32,
    y: f32,
    max_width: f32,
    fonts: &mut FontSet,
    base_style: Style,
) -> Result<(Vec<TextRun>, f32), Box<dyn std::error::Error>> {
    let mut chars = Vec::new();
    for node in nodes {
        flatten(node, base_style, &mut chars);
    }

    let mut words: Vec<(String, Style)> = Vec::new();
    let mut current = String::new();
    let mut current_style = base_style;

    for (c, style) in chars {
        if c.is_whitespace() {
            if !current.is_empty() {
                words.push((std::mem::take(&mut current), current_style));
            }
        } else {
            if current.is_empty() {
                current_style = style;
            }
            current.push(c);
        }
    }
    if !current.is_empty() {
        words.push((current, current_style));
    }

    let mut runs = Vec::new();

    if words.is_empty() {
        return Ok((runs, 0.0));
    }

    let mut cursor_x = x;
    let mut cursor_y = y;
    let mut line_height = base_style.size;

    for (word, style) in words {
        let (word_width, _) = fonts
            .get(style.bold, style.italic)
            .measure(&word, style.size)?;
        let (space_width, _) = fonts
            .get(style.bold, style.italic)
            .measure(" ", style.size)?;

        if style.size > line_height {
            line_height = style.size;
        }

        if cursor_x > x && cursor_x + space_width + word_width > x + max_width {
            cursor_x = x;
            cursor_y += line_height;
        } else if cursor_x > x {
            cursor_x += space_width;
        }

        runs.push(TextRun::new(word, cursor_x, cursor_y, style));

        cursor_x += word_width;
    }

    let used_height = (cursor_y - y) + line_height;

    Ok((runs, used_height))
}
