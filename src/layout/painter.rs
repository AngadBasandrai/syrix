use super::layout_box::LayoutBox;
use crate::graphics::renderer::Renderer;

pub fn paint(layout: &LayoutBox, renderer: &Renderer) {
    renderer.draw_rect(layout.x(), layout.y(), layout.width(), layout.height());

    for child in layout.children() {
        paint(child, renderer);
    }
}
