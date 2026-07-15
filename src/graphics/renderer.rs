use super::color::Color;
use super::mesh::Mesh;
use super::shader::Shader;
use super::texture::Texture;

pub struct Renderer {
    shader: Shader,
    rectangle: Mesh,
    width: f32,
    height: f32,
}

impl Renderer {
    pub fn new(width: f32, height: f32) -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        Ok(Self {
            shader: Shader::new("shaders/triangle.vert", "shaders/triangle.frag")?,
            rectangle: Mesh::unit_square(),
            width: width,
            height: height,
        })
    }

    fn pixels_to_ndc(&self, x: f32, y: f32) -> (f32, f32) {
        (
            (x / (self.width / 2.0)) - 1.0,
            1.0 - (y / (self.height / 2.0)),
        )
    }

    pub fn draw_rect(&self, x: f32, y: f32, width: f32, height: f32, color: &Color) {
        self.shader.use_program();

        let (ndc_x, ndc_y) = self.pixels_to_ndc(x, y);

        let ndc_width = (2.0 * width) / self.width;
        let ndc_height = (2.0 * height) / self.height;

        self.shader.set_vec2("uPosition", ndc_x, ndc_y);
        self.shader.set_vec2("uSize", ndc_width, -ndc_height);
        self.shader
            .set_vec3("uColor", color.r(), color.g(), color.b());

        self.rectangle.draw();
    }
    pub fn draw_texture(
        &self,
        texture: &Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: &Color,
    ) {
        self.shader.use_program();

        let (ndc_x, ndc_y) = self.pixels_to_ndc(x, y);

        let ndc_width = (2.0 * width) / self.width;
        let ndc_height = (2.0 * height) / self.height;

        self.shader.set_vec2("uPosition", ndc_x, ndc_y);
        self.shader.set_vec2("uSize", ndc_width, -ndc_height);

        self.shader
            .set_vec3("uColor", color.r(), color.g(), color.b());

        texture.bind(0);
        self.shader.set_int("uTexture", 0);

        self.rectangle.draw();
    }
}
