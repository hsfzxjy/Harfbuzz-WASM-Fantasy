mod maze;
#[macro_use]
mod prelude;
use std::borrow::Cow;

use harfbuzz_wasm::{debug, Font, Glyph, GlyphBuffer};
use maze::MAZE_RENDERER;
use prelude::*;
use wasm_bindgen::prelude::*;

make_registry!(MAZE_RENDERER);

#[wasm_bindgen]
pub fn shape(
    _shape_plan: u32,
    font_ref: u32,
    buf_ref: u32,
    _features: u32,
    _num_features: u32,
) -> i32 {
    let font = Font::from_ref(font_ref);
    let mut buffer = GlyphBuffer::from_ref(buf_ref);
    let mut glyph_lists = Vec::<Cow<[Glyph]>>::new();

    let mut glyphs: &[Glyph] = &buffer.glyphs;
    'OUTER: while !glyphs.is_empty() {
        for (i, g) in glyphs.iter().enumerate() {
            let renderer = match REGISTRY.get_renderer(g.codepoint) {
                Some(r) => r,
                _ => continue,
            };
            let (previous, current) = glyphs.split_at(i);
            let result = match renderer(&font, current) {
                Some(r) => r,
                _ => continue,
            };
            glyph_lists.extend([Cow::Borrowed(previous), Cow::Owned(result.new_glyphs)]);
            glyphs = result.rest;
            continue 'OUTER;
        }
        break;
    }

    if !glyph_lists.is_empty() {
        buffer.glyphs = glyph_lists
            .iter()
            .chain(std::iter::once(&Cow::Borrowed(glyphs)))
            .map(|s| match s {
                Cow::Borrowed(s) => s.into_iter(),
                Cow::Owned(s) => s.into_iter(),
            })
            .flatten()
            .map(Glyph::clone)
            .collect::<Vec<_>>();
    }

    for g in &mut buffer.glyphs {
        let cp = g.codepoint;
        g.codepoint = font.get_glyph(g.codepoint, 0);
        if cp < 0xF0000 {
            g.x_advance = font.get_glyph_h_advance(g.codepoint);
        }
    }
    1
}
