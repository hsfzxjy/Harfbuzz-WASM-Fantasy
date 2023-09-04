<h1 align=center>Harfbuzz-WASM-Fantasy</h1>

<p align=center><b>:bulb: STUPID stuffs built by abusing <a href="https://github.com/harfbuzz/harfbuzz/blob/main/docs/wasm-shaper.md">Harfbuzz WASM Shaper</a>.</b></p>

|      [Bad Apple][link-bad-apple]       |    [Maze Game][link-maze]    |
| :------------------------------------: | :--------------------------: |
| [![][cover-bad-apple]](link-bad-apple) | [![][cover-maze]][link-maze] |

[link-bad-apple]: https://github.com/hsfzxjy/Bad-Apple-Font
[cover-bad-apple]: https://github.com/hsfzxjy/Harfbuzz-WASM-Fantasy/assets/4702188/04241bd6-1525-4b9a-bc50-db6bfe93ec5a
[link-maze]: #maze-game
[cover-maze]: https://github.com/hsfzxjy/Harfbuzz-WASM-Fantasy/assets/4702188/97dbd2c4-ec9b-4249-b4ce-a529709f1ade

## Prerequisites

First, clone this project to your local machine and update all sub-modules:

```bash
git clone https://github.com/hsfzxjy/Bad-Apple-Font
git submodule update --init --recursive
```

Before we start, make sure the following commands are globally available:

```plaintext
python3 cmake g++ wget tar ffmpeg cargo rustup
```

Besides, `fontmake` would be used for font generation:

```bash
python3 -m pip install fontmake
```

## Building & Hacking

To build this project you simply need to run `make`. Afterwards, execute `scripts/hack` to start a hijacked gedit.

---

## Gallery

### Maze-Game

[[Source Code](./src/maze/)]

The Maze-Game can be triggered by typing the following pattern:

```plaintext
maze#<size>#<seed>#
```

where `5 <= size <= 20` defines the size of maze, and `0 <= seed <= 99` is a seed for RNG. For example, `maze#5#3#` generates a maze with 5x5 grid.

All mazes start from the bottom-left corner and target the top-right corner. The player can use <kbd>W</kbd><kbd>S</kbd><kbd>A</kbd><kbd>D</kbd> for movement.

https://user-images.githubusercontent.com/4702188/265505370-6d528e98-63ec-4b56-9f33-aabd654905e7.mp4

## License

This project is licensed under the [MIT LICENSE](./LICENSE). Copyright (c) 2023 hsfzxjy.
