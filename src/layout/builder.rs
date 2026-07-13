use crate::dom::structures::{ElementNode, Node};
use crate::layout::layout_box::LayoutBox;

pub fn build_layout(
    node: &Node,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Option<(LayoutBox, f32)> {
    if let Some(elem) = node.as_element() {
        Some(build_element(elem, x, y, width, height))
    } else {
        None
    }
}

fn build_element(
    element: &ElementNode,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> (LayoutBox, f32) {
    let mut children = Vec::new();
    let mut current_y = 20.0;
    for child in element.children() {
        if let Some(layout) = build_layout(child, x + 20.0, y + current_y, width - 20.0, 20.0) {
            children.push(layout.0);
            current_y += layout.1;
        }
    }
    (
        LayoutBox::new(element.tag().into(), x, y, width, height, children),
        current_y,
    )
}
