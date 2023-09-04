use harfbuzz_wasm::{Font, Glyph};

pub type RenderFn = for<'a, 'b> fn(font: &'a Font, glyphs: &'b [Glyph]) -> Option<RenderResult<'b>>;

pub struct RenderResult<'a> {
    pub new_glyphs: Vec<Glyph>,
    pub rest: &'a [Glyph],
}

#[derive(Copy, Clone)]
pub struct Renderer {
    pub hint: u32,
    pub func: RenderFn,
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! make_registry {
    ($($item:tt),+) => {
        struct Registry {
            renderers: [Renderer; count!($($item)+)],
            char_to_index: [u8; 128],
        }

        const REGISTRY: Registry = {
            let mut r = Registry {
                renderers: [$($item),+],
                char_to_index: [0u8;128]
            };
            let mut idx = 0;
            $(make_registry!(_set_index r idx $item));+;
            let _ = idx;
            r
        };

        impl Registry {
            fn get_renderer(&self, ch: u32) -> Option<RenderFn> {
                let idx = self.char_to_index[ch as usize] as usize;
                (idx != 255).then(|| self.renderers[idx].func)
            }
        }
    };
    (_set_index $r:tt $idx:tt $item:tt) => {
        $r.char_to_index[$item.hint as usize] = $idx;
        $idx += 1;
    }
}
