use crate::dom::structures::{ElementNode, Node};
use crate::layout::layout_box::LayoutBox;

pub fn build_layout(node: &Node, x: f32, y: f32, width: f32, height: f32) -> Option<LayoutBox> {
    if let Some(elem) = node.as_element() {
        Some(build_element(elem, x, y, width, height))
    } else {
        None
    }
}

fn build_element(element: &ElementNode, x: f32, y: f32, width: f32, height: f32) -> LayoutBox {
    let mut children = Vec::new();
    let mut current_y = 20.0;
    for child in element.children() {
        if let Some(layout) = build_layout(child, x + 20.0, y + current_y, width - 40.0, 20.0) {
            children.push(layout);
            current_y += 20.0;
        }
    }
    LayoutBox::new(element.tag().into(), x, y, width, height, children)
}
