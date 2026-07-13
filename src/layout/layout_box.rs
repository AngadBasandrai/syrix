pub struct LayoutBox {
    tag: String,

    x: f32,
    y: f32,
    width: f32,
    height: f32,

    children: Vec<LayoutBox>,
}

impl LayoutBox {
    pub fn new(
        tag: String,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        children: Vec<LayoutBox>,
    ) -> Self {
        Self {
            tag,
            x,
            y,
            width,
            height,
            children,
        }
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
    pub fn children(&self) -> &[LayoutBox] {
        &self.children
    }
}
