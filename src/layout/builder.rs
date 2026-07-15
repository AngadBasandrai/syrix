use crate::dom::structures::{ElementNode, Node};
use crate::graphics::text::FontSet;

use super::display::{Display, default_display};
use super::inline::{is_inline_node, layout_inline};
use super::layout_box::LayoutBox;
use super::style::block_style;

pub fn build_layout<'a>(
    node: &'a Node,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    fonts: &mut FontSet,
) -> Option<(LayoutBox<'a>, f32)> {
    match node {
        Node::Element(element) => Some(build_element(element, x, y, width, height, fonts)),

        Node::Text(_) => None,
    }
}

fn build_element<'a>(
    element: &'a ElementNode,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    fonts: &mut FontSet,
) -> (LayoutBox<'a>, f32) {
    let mut children = Vec::new();
    let mut text_runs = Vec::new();
    let mut current_y = 20.0;
    let mut inline_group: Vec<&Node> = Vec::new();

    let style = block_style(element.tag());

    for child in element.children() {
        if let Node::Element(child_element) = child {
            if default_display(child_element.tag()) == Display::None {
                continue;
            }
        }

        if is_inline_node(child) {
            inline_group.push(child);
            continue;
        }

        if !inline_group.is_empty() {
            if let Ok((runs, used_height)) = layout_inline(
                &inline_group,
                x + 20.0,
                y + current_y,
                width - 20.0,
                fonts,
                style,
            ) {
                text_runs.extend(runs);
                current_y += used_height;
            }
            inline_group.clear();
        }

        if let Some((layout, used_height)) =
            build_layout(child, x + 20.0, y + current_y, width - 20.0, 20.0, fonts)
        {
            children.push(layout);
            current_y += used_height;
        }
    }

    if !inline_group.is_empty() {
        if let Ok((runs, used_height)) = layout_inline(
            &inline_group,
            x + 20.0,
            y + current_y,
            width - 20.0,
            fonts,
            style,
        ) {
            text_runs.extend(runs);
            current_y += used_height;
        }
    }

    let display = default_display(element.tag());

    (
        LayoutBox::new(element, display, x, y, width, height, children, text_runs),
        current_y,
    )
}
