use super::color::Color;
use super::display_list::DisplayCommand;
use super::mesh::Mesh;
use super::shader::Shader;
use super::text::FontSet;
use super::texture::Texture;

pub struct Renderer {
    shader: Shader,
    rectangle: Mesh,
    solid: Texture,
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
            solid: Texture::from_grayscale(1, 1, &[255]),
            width,
            height,
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

        self.solid.bind(0);
        self.shader.set_int("uTexture", 0);

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

    pub fn draw_text(
        &self,
        fonts: &mut FontSet,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        bold: bool,
        italic: bool,
        color: &Color,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let font = fonts.get(bold, italic);

        let mut curr_x = x;
        let mut curr_y = y;

        for c in text.chars() {
            match c {
                '\n' => {
                    curr_y += size;
                    curr_x = x;
                }

                _ => {
                    let glyph = font.glyph(c, size)?;

                    if let Some(texture) = glyph.texture() {
                        self.draw_texture(
                            texture,
                            curr_x + glyph.bearing_x(),
                            curr_y + glyph.bearing_y(),
                            glyph.width(),
                            glyph.height(),
                            color,
                        );
                    }

                    curr_x += glyph.advance();
                }
            }
        }

        Ok(())
    }

    pub fn render(
        &self,
        commands: &[DisplayCommand],
        fonts: &mut FontSet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for command in commands {
            match command {
                DisplayCommand::Rect {
                    x,
                    y,
                    width,
                    height,
                    color,
                } => {
                    self.draw_rect(*x, *y, *width, *height, color);
                }

                DisplayCommand::Text {
                    text,
                    x,
                    y,
                    size,
                    bold,
                    italic,
                    color,
                } => {
                    self.draw_text(fonts, text, *x, *y, *size, *bold, *italic, color)?;
                }
            }
        }

        Ok(())
    }
}
