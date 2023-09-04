use harfbuzz_wasm::{Font, Glyph};
use tiny_rng::{Rand, Rng};

use crate::prelude::*;

use self::map::{Map, Position};

mod map;

fn render_maze<'a, 'b>(font: &'a Font, glyphs: &'b [Glyph]) -> Option<RenderResult<'b>> {
    let pixel_size = get_pixel_size(font);

    let mut scanner = Scanner::new(glyphs);
    scanner.scan_str("maze#")?;
    let size: i32 = scanner.scan_uint(5, 20)? as i32;
    scanner.scan_str("#")?;
    let seed = scanner.scan_uint(0, 99)?;
    scanner.scan_str("#")?;

    let mut rng = Rng::from_seed(seed as u64);
    let map = Map::generate_prim(size, size, Position(0, 0), &mut rng);
    let mut ng = GlyphVecBuilder::new(size as usize, pixel_size);

    for i in 0..(size - 1) {
        ng.push(UNICODE_VLINE, 0, i + 1);
        ng.push(UNICODE_VLINE, size, i);
    }

    for i in 0..(size) {
        ng.push(UNICODE_HLINE, i, 0);
        ng.push(UNICODE_HLINE, i, size);
    }

    for i in 0..(size) {
        for j in 0..(size) {
            if i < size - 1 && map.is_above(&Position(i, j)) {
                ng.push(UNICODE_HLINE, j, i + 1);
            }

            if j < size - 1 && map.is_right(&Position(i, j)) {
                ng.push(UNICODE_VLINE, j + 1, i);
            }
        }
    }

    let mut pos = Position(0, 0);
    let dest = Position(size - 1, size - 1);
    while let Some(key) = scanner.peek_char() {
        match key {
            KEY_UP => {
                if pos.0 < size - 1 && !map.is_above(&pos) {
                    pos.0 += 1;
                }
            }
            KEY_DOWN => {
                if pos.0 != 0 && !map.is_below(&pos) {
                    pos.0 -= 1;
                }
            }
            KEY_LEFT => {
                if pos.1 != 0 && !map.is_left(&pos) {
                    pos.1 -= 1;
                }
            }
            KEY_RIGHT => {
                if pos.1 < size - 1 && !map.is_right(&pos) {
                    pos.1 += 1;
                }
            }
            _ => break,
        }
        scanner.advance_one();
        if pos == dest {
            break;
        }
    }
    ng.push(UNICODE_SMALLPIXEL, pos.1 as i32, pos.0 as i32);
    if pos == dest {
        ng.push_str("WIN!");
    }
    Some(scanner.into_result(ng.into()))
}

pub const MAZE_RENDERER: Renderer = Renderer {
    func: render_maze,
    hint: 'm' as u32,
};
