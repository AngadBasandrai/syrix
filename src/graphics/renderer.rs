use super::mesh::Mesh;
use super::shader::Shader;

pub struct Renderer {
    shader: Shader,
    width: f32,
    height: f32,
}

impl Renderer {
    pub fn new(width: f32, height: f32) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            shader: Shader::new("shaders/triangle.vert", "shaders/triangle.frag")?,
            width: width,
            height: height,
        })
    }
    fn draw_mesh(&self, mesh: &Mesh) {
        self.shader.use_program();
        mesh.draw();
    }

    fn pixels_to_ndc(&self, x: f32, y: f32) -> (f32, f32) {
        (
            (x / (self.width / 2.0)) - 1.0,
            1.0 - (y / (self.height / 2.0)),
        )
    }

    pub fn draw_rect(&self, x: f32, y: f32, width: f32, height: f32) {
        let right = x + width;
        let bottom = y + height;
        let (top_left_x, top_left_y) = self.pixels_to_ndc(x, y);
        let (top_right_x, top_right_y) = self.pixels_to_ndc(right, y);
        let (bottom_left_x, bottom_left_y) = self.pixels_to_ndc(x, bottom);
        let (bottom_right_x, bottom_right_y) = self.pixels_to_ndc(right, bottom);
        let vertices = [
            top_left_x,
            top_left_y,
            0.0,
            top_right_x,
            top_right_y,
            0.0,
            bottom_left_x,
            bottom_left_y,
            0.0,
            bottom_left_x,
            bottom_left_y,
            0.0,
            top_right_x,
            top_right_y,
            0.0,
            bottom_right_x,
            bottom_right_y,
            0.0,
        ];
        let mesh = Mesh::new(&vertices);

        self.draw_mesh(&mesh);
    }
}
