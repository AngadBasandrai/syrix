use std::rc::Rc;

use crate::dom::structures::Node;
use crate::graphics::text::FontSet;

use super::display::{Display, default_display};
use super::style::{Style, inline_style};

pub struct TextRun {
    text: String,
    x: f32,
    y: f32,
    width: f32,
    style: Style,
    href: Option<Rc<str>>,
}

impl TextRun {
    pub fn new(
        text: String,
        x: f32,
        y: f32,
        width: f32,
        style: Style,
        href: Option<Rc<str>>,
    ) -> Self {
        Self {
            text,
            x,
            y,
            width,
            style,
            href,
        }
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

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn style(&self) -> Style {
        self.style
    }

    pub fn href(&self) -> Option<&str> {
        self.href.as_deref()
    }
}

pub fn is_inline_node(node: &Node) -> bool {
    match node {
        Node::Text(_) => true,
        Node::Element(element) => matches!(default_display(element.tag()), Display::Inline),
    }
}

fn flatten(
    node: &Node,
    style: Style,
    href: Option<Rc<str>>,
    out: &mut Vec<(char, Style, Option<Rc<str>>)>,
) {
    match node {
        Node::Text(text) => {
            for c in text.content().chars() {
                out.push((c, style, href.clone()));
            }
        }
        Node::Element(element) => {
            let style = inline_style(element.tag(), style);

            let href = if element.tag() == "a" {
                element
                    .attribute("href")
                    .map(|value| Rc::from(value.as_str()))
            } else {
                href
            };

            for child in element.children() {
                flatten(child, style, href.clone(), out);
            }
        }
    }
}

fn same_run(
    a_style: &Style,
    a_href: &Option<Rc<str>>,
    b_style: &Style,
    b_href: &Option<Rc<str>>,
) -> bool {
    a_style.size == b_style.size
        && a_style.bold == b_style.bold
        && a_style.italic == b_style.italic
        && a_style.color.r() == b_style.color.r()
        && a_style.color.g() == b_style.color.g()
        && a_style.color.b() == b_style.color.b()
        && a_href.as_deref() == b_href.as_deref()
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
        flatten(node, base_style, None, &mut chars);
    }

    let mut words: Vec<Vec<(char, Style, Option<Rc<str>>)>> = Vec::new();
    let mut current = Vec::new();

    for entry in chars {
        if entry.0.is_whitespace() {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
        } else {
            current.push(entry);
        }
    }
    if !current.is_empty() {
        words.push(current);
    }

    let mut runs = Vec::new();

    if words.is_empty() {
        return Ok((runs, 0.0));
    }

    let mut cursor_x = x;
    let mut cursor_y = y;
    let mut line_height = base_style.size;

    let (space_width, _) = fonts
        .get(base_style.bold, base_style.italic)
        .measure(" ", base_style.size)?;

    for word in words {
        let mut segments: Vec<(String, Style, Option<Rc<str>>)> = Vec::new();

        for (c, style, href) in word {
            match segments.last_mut() {
                Some(last) if same_run(&last.1, &last.2, &style, &href) => {
                    last.0.push(c);
                }
                _ => segments.push((c.to_string(), style, href)),
            }
        }

        let mut seg_widths = Vec::with_capacity(segments.len());
        let mut word_width = 0.0;
        let mut word_max_size = 0.0f32;

        for (text, style, _) in &segments {
            let (width, _) = fonts
                .get(style.bold, style.italic)
                .measure(text, style.size)?;
            seg_widths.push(width);
            word_width += width;
            word_max_size = word_max_size.max(style.size);
        }

        if word_max_size > line_height {
            line_height = word_max_size;
        }

        if cursor_x > x && cursor_x + space_width + word_width > x + max_width {
            cursor_x = x;
            cursor_y += line_height;
        } else if cursor_x > x {
            cursor_x += space_width;
        }

        for ((text, style, href), width) in segments.into_iter().zip(seg_widths) {
            runs.push(TextRun::new(text, cursor_x, cursor_y, width, style, href));
            cursor_x += width;
        }
    }

    let used_height = (cursor_y - y) + line_height;

    Ok((runs, used_height))
}
