use harfbuzz_wasm::Glyph;

use super::RenderResult;

pub struct Scanner<'a> {
    glyphs: &'a [Glyph],
}

impl<'a> Scanner<'a> {
    pub fn new(glyphs: &'a [Glyph]) -> Self {
        Self { glyphs }
    }
    pub fn scan_str(&mut self, v: &str) -> Option<()> {
        if v.len() > self.glyphs.len() {
            return None;
        }
        let (a, b) = self.glyphs.split_at(v.len());
        if a.iter()
            .zip(v.as_bytes())
            .all(|(g, c)| g.codepoint == *c as u32)
        {
            self.glyphs = b;
            return Some(());
        }
        None
    }

    pub fn scan_uint(&mut self, min: u32, max: u32) -> Option<u32> {
        let mut x = 0u32;
        loop {
            let first_digit = self
                .glyphs
                .first()
                .map(|g| char::from_u32(g.codepoint))
                .flatten()
                .map(|c| c.to_digit(10))
                .flatten();
            if let Some(d) = first_digit {
                let y = x * 10 + d;
                if y > max {
                    break;
                }
                x = y;
            } else {
                break;
            }
            self.advance_one();
        }
        if min <= x && x <= max {
            Some(x)
        } else {
            None
        }
    }

    pub fn peek_char(&mut self) -> Option<u32> {
        self.glyphs.first().map(|g| g.codepoint)
    }

    pub fn advance_one(&mut self) -> Option<()> {
        self.glyphs = self.glyphs.split_first()?.1;
        Some(())
    }

    pub fn into_result(self, new_glyphs: Vec<Glyph>) -> RenderResult<'a> {
        RenderResult {
            new_glyphs,
            rest: self.glyphs,
        }
    }
}
