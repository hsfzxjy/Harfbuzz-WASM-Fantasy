from typing import List, Tuple
from pathlib import Path
from dataclasses import dataclass
import textwrap

layerIds = [
    "EB62C368-BB48-47C0-A466-DA94204A0DE4",
    "2D111E5E-352D-4630-9162-BCEF010D26CF",
    "A6F5E7A4-47EF-4F55-BF18-0ACACB57A720",
    "1615C59E-33DE-4B88-B2CC-A48C7D56A125",
    "90D50F69-F6F7-4633-9462-ED0440D3064F",
    "12552169-0282-4A31-82CC-880002E60A2C",
    "44982FF8-7820-4F75-948C-97259056F879",
    "84B0614D-4B2F-419D-B7F2-5B7DD4CAD23C",
    "5D382461-81E1-4DF2-8677-8A6F02AD5363",
    "3B8883BF-F483-4B97-AC7C-CABF8B821093",
    "F5F01D5B-AEA3-4AC3-B3D8-92550D1582D7",
    "A390C1A2-E540-434D-BB9E-4BCEE875BA63",
]


@dataclass
class Glyph:
    glyphname: str
    unicode: int
    nodes: List[Tuple[int, int, str]]

    def as_text(self) -> str:
        def make_node(node):
            return f"({node[0]}, {node[1]}, {node[2]})"

        def make_layer(layerId):
            nodes = ", ".join(map(make_node, self.nodes))
            return textwrap.dedent(
                f"""{{
                layerId = "{layerId}"; 
                shapes = ({{ 
                    closed = 1; 
                    nodes = ({nodes}); 
                }}); 
            }}"""
            )

        layers = ", ".join(map(make_layer, layerIds))
        return textwrap.dedent(
            f"""
            {{
                glyphname = {self.glyphname};
                layers = ({layers});
                unicode = {self.unicode};
            }},
            """
        )


new_glyphs = [
    Glyph(
        "Pixel",
        0xF0001,
        [(0, 0, "ls"), (0, 50, "ls"), (50, 50, "ls"), (50, 0, "ls")],
    ),
    Glyph(
        "HLine",
        0xF0002,
        [(0, 5, "ls"), (50, 5, "ls"), (50, -5, "ls"), (0, -5, "ls")],
    ),
    Glyph(
        "VLine",
        0xF0003,
        [(5, 0, "ls"), (5, 50, "ls"), (-5, 50, "ls"), (-5, 0, "ls")],
    ),
    Glyph(
        "SmallPixel",
        0xF0004,
        [(15, 15, "ls"), (15, 35, "ls"), (35, 35, "ls"), (35, 15, "ls")],
    ),
    Glyph(
        "Pixel20",
        0xF0005,
        [(0, 0, "ls"), (0, 20, "ls"), (20, 20, "ls"), (20, 0, "ls")],
    ),    
]

glyphs_src = Path("bin") / "glyphs_src"
content = (glyphs_src / "Origin.glyphs").read_text()
glyphs = "".join(g.as_text() for g in new_glyphs)
content = content.replace("glyphs = (\n", "glyphs = (\n" + glyphs)
content = content.replace("DM Sans 9pt", "HB_WASM_Fantasy")
(glyphs_src / "Manipulated.glyphs").write_text(content)
