# Infinite Minesweeper

Minesweeper with an infinite\* size.

You can left-click to uncover and right-click to set a flag. Drag while holding Ctrl to scroll through the map.

\* Until you run out of memory or `isize` overflows. This may be resolved in the future.

## Building

```bash
git clone https://github.com/MattTheNub/infinite-minesweeper
cd infinite-minesweeper
cargo build --release
```

The binary can then be found in `target/release/`
