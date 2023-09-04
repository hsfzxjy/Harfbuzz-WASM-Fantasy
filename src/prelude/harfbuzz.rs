use harfbuzz_wasm::{Font, Glyph};

use super::UNICODE_PIXEL;

pub fn get_pixel_size(font: &Font) -> i32 {
    let pixel_id = font.get_glyph(UNICODE_PIXEL, 0);
    -font.get_glyph_extents(pixel_id).height
}

pub struct GlyphVecBuilder {
    vec: Vec<Glyph>,
    n: i32,
    pixel_size: i32,
}

impl GlyphVecBuilder {
    pub fn new(n: usize, pixel_size: i32) -> Self {
        Self {
            vec: Vec::with_capacity(n * (2 * n)),
            n: n as i32,
            pixel_size,
        }
    }
    fn x_advance(&self) -> i32 {
        self.pixel_size * (self.n + 5)
    }
    fn x_base(&self) -> i32 {
        self.pixel_size * 4
    }
    fn y_base(&self) -> i32 {
        -self.pixel_size * 4
    }
    pub fn push(&mut self, code: u32, x: i32, y: i32) {
        self.vec.last_mut().map(|x| x.x_advance = 0);
        self.vec.push(Glyph {
            codepoint: code,
            cluster: 0,
            x_advance: self.x_advance(),
            y_advance: 0,
            x_offset: x * self.pixel_size + self.x_base(),
            y_offset: y * self.pixel_size + self.y_base(),
            flags: 0,
        })
    }
    pub fn push_str(&mut self, s: &str) {
        self.vec.extend(s.chars().map(|c| Glyph {
            codepoint: c.into(),
            cluster: 0,
            x_advance: 0,
            y_advance: 0,
            x_offset: 0,
            y_offset: 0,
            flags: 0,
        }))
    }
}

impl Into<Vec<Glyph>> for GlyphVecBuilder {
    fn into(self) -> Vec<Glyph> {
        self.vec
    }
}
