use super::layout_box::LayoutBox;
use crate::graphics::{color::Color, renderer::Renderer};

pub fn paint(layout: &LayoutBox, renderer: &Renderer) {
    let color = match layout.tag() {
        "h1" => Color::new(0.0, 1.0, 0.0),
        "p" => Color::new(1.0, 1.0, 0.0),
        "body" => Color::new(0.5, 0.5, 0.5),
        "div" => Color::new(0.0, 0.0, 1.0),
        _ => Color::new(1.0, 1.0, 1.0),
    };

    renderer.draw_rect(
        layout.x(),
        layout.y(),
        layout.width(),
        layout.height(),
        &color,
    );

    for child in layout.children() {
        paint(child, renderer);
    }
}

pub fn print_layout(layout: &LayoutBox, depth: usize) {
    println!(
        "{}{} ({}, {}) {}x{}",
        "\t".repeat(depth),
        layout.tag(),
        layout.x(),
        layout.y(),
        layout.width(),
        layout.height(),
    );

    for child in layout.children() {
        print_layout(child, depth + 1);
    }
}
