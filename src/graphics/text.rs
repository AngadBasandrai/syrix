use std::{error::Error, fs};

use ab_glyph::{Font as AbFont, FontArc, ScaleFont};

use super::texture::Texture;

pub struct Font {
    font: FontArc,
}

impl Font {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let bytes = fs::read(path)?;
        let font = FontArc::try_from_vec(bytes)?;

        Ok(Self { font })
    }
}

pub struct Glyph {
    texture: Texture,

    width: f32,
    height: f32,

    advance: f32,
}

impl Glyph {
    pub fn new(font: &Font, character: char, size: f32) -> Result<Self, Box<dyn Error>> {
        let glyph_id = font.font.glyph_id(character);
        let glyph = glyph_id.with_scale(size);

        let advance = font.font.as_scaled(size).h_advance(glyph_id);

        let outlined = font
            .font
            .outline_glyph(glyph)
            .ok_or("Failed to outline glyph")?;

        let bounds = outlined.px_bounds();

        let width = bounds.width().ceil() as usize;
        let height = bounds.height().ceil() as usize;

        let mut bitmap = vec![0u8; width * height];

        outlined.draw(|x, y, coverage| {
            bitmap[y as usize * width + x as usize] = (coverage * 255.0) as u8;
        });

        let texture = Texture::from_grayscale(width, height, &bitmap);

        Ok(Self {
            texture,
            width: width as f32,
            height: height as f32,
            advance,
        })
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn advance(&self) -> f32 {
        self.advance
    }
}
