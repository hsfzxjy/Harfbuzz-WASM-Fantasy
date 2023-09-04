WASM=pkg/harfbuzz_fantasy_bg.wasm

GlyphsDir=bin/glyphs_src

Glyphs_Origin=$(GlyphsDir)/Origin.glyphs
Glyphs_Manipulated=$(GlyphsDir)/Manipulated.glyphs
TTF_Raw=bin/HB_WASM_Fantasy-Thin.ttf
TTF_Final=bin/HB_WASM_Fantasy.ttf

$(TTF_Final): $(WASM) $(TTF_Raw)
	bin/otfsurgeon -i $(TTF_Raw) add -o $(TTF_Final) Wasm < $(WASM)

$(TTF_Raw): $(Glyphs_Manipulated)
	fontmake -o ttf -g $(Glyphs_Manipulated) --output-dir=bin -i 'HB_WASM_Fantasy Thin'

$(Glyphs_Origin):
	mkdir -p $(GlyphsDir)
	wget "https://raw.githubusercontent.com/googlefonts/dm-fonts/d0520ba03bd780f5dccb3024854463d44f699b78/Sans/Source/DMSans.glyphs" -O $(Glyphs_Origin)

$(Glyphs_Manipulated): $(Glyphs_Origin) scripts/make-font.py
	python3 scripts/make-font.py

$(WASM): $(wildcard src/**/*) $(wildcard src/*)
	wasm-pack build --target web

.PHONY: harfbuzz
harfbuzz:
	scripts/build_harfbuzz.sh