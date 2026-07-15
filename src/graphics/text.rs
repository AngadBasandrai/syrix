use std::{collections::HashMap, error::Error, fs};

use ab_glyph::{Font as AbFont, FontArc, ScaleFont};

use super::texture::Texture;

pub struct Font {
    font: FontArc,
    glyphs: HashMap<(char, u32), Glyph>,
}

impl Font {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let bytes = fs::read(path)?;
        let font = FontArc::try_from_vec(bytes)?;

        Ok(Self {
            font,
            glyphs: HashMap::new(),
        })
    }

    pub fn glyph(&mut self, character: char, size: f32) -> Result<&Glyph, Box<dyn Error>> {
        let key = (character, size.round() as u32);

        if !self.glyphs.contains_key(&key) {
            let glyph = Glyph::new(self, character, size)?;
            self.glyphs.insert(key, glyph);
        }
        Ok(self.glyphs.get(&key).unwrap())
    }

    pub fn measure(&mut self, text: &str, size: f32) -> Result<(f32, f32), Box<dyn Error>> {
        let mut width = 0.0;

        for c in text.chars() {
            let glyph = self.glyph(c, size)?;
            width += glyph.advance();
        }

        Ok((width, size))
    }
}

pub struct FontSet {
    regular: Font,
    bold: Font,
    italic: Font,
    bold_italic: Font,
}

impl FontSet {
    pub fn load(dir: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            regular: Font::load(&format!("{dir}/COMIC.TTF"))?,
            bold: Font::load(&format!("{dir}/COMICBD.TTF"))?,
            italic: Font::load(&format!("{dir}/COMICI.TTF"))?,
            bold_italic: Font::load(&format!("{dir}/COMICZ.TTF"))?,
        })
    }

    pub fn get(&mut self, bold: bool, italic: bool) -> &mut Font {
        match (bold, italic) {
            (true, true) => &mut self.bold_italic,
            (true, false) => &mut self.bold,
            (false, true) => &mut self.italic,
            (false, false) => &mut self.regular,
        }
    }
}

pub struct Glyph {
    texture: Option<Texture>,

    width: f32,
    height: f32,

    bearing_x: f32,
    bearing_y: f32,

    advance: f32,
}

impl Glyph {
    pub fn new(font: &Font, character: char, size: f32) -> Result<Self, Box<dyn Error>> {
        let glyph_id = font.font.glyph_id(character);
        let glyph = glyph_id.with_scale(size);

        let advance = font.font.as_scaled(size).h_advance(glyph_id);

        if let Some(outlined) = font.font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();

            let width = bounds.width().ceil() as usize;
            let height = bounds.height().ceil() as usize;
            let bounds = outlined.px_bounds();

            let mut bitmap = vec![0u8; width * height];

            outlined.draw(|x, y, coverage| {
                bitmap[y as usize * width + x as usize] = (coverage * 255.0) as u8;
            });

            let texture = Texture::from_grayscale(width, height, &bitmap);

            Ok(Self {
                texture: Some(texture),
                width: width as f32,
                height: height as f32,
                bearing_x: bounds.min.x,
                bearing_y: bounds.min.y,
                advance,
            })
        } else {
            Ok(Self {
                texture: None,
                width: 0.0,
                height: 0.0,
                bearing_x: 0.0,
                bearing_y: 0.0,
                advance,
            })
        }
    }

    pub fn texture(&self) -> Option<&Texture> {
        self.texture.as_ref()
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn bearing_x(&self) -> f32 {
        self.bearing_x
    }

    pub fn bearing_y(&self) -> f32 {
        self.bearing_y
    }

    pub fn advance(&self) -> f32 {
        self.advance
    }
}
