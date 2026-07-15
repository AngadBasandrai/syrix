use crate::dom::structures::ElementNode;

use super::display::Display;
use super::inline::TextRun;

pub struct LayoutBox<'a> {
    element: &'a ElementNode,
    display: Display,

    x: f32,
    y: f32,
    width: f32,
    height: f32,

    children: Vec<LayoutBox<'a>>,
    text_runs: Vec<TextRun>,
}

impl<'a> LayoutBox<'a> {
    pub fn new(
        element: &'a ElementNode,
        display: Display,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        children: Vec<LayoutBox<'a>>,
        text_runs: Vec<TextRun>,
    ) -> Self {
        Self {
            element,
            display,
            x,
            y,
            width,
            height,
            children,
            text_runs,
        }
    }

    pub fn element(&self) -> &'a ElementNode {
        self.element
    }

    pub fn display(&self) -> Display {
        self.display
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

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn children(&self) -> &[LayoutBox<'a>] {
        &self.children
    }

    pub fn text_runs(&self) -> &[TextRun] {
        &self.text_runs
    }
}
