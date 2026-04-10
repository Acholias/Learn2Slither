# Learn2Slither

[French version / Version francaise](README.fr.md)

Learn2Slither is a small Snake game written in Rust with a graphical interface powered by Macroquad.

## Features

- 15x15 board
- Real-time keyboard controls
- Green apples grow the snake
- Red apples shrink the snake
- Collision detection (walls and body)
- Restart support after game over

## Architecture

```text
learn2slither/
├── src/
│   ├── main.rs      # Game loop, input handling, timing, restart flow
│   ├── board.rs     # Board state, apple spawning, collision rules, step logic
│   ├── snake.rs     # Snake model, movement, growth/shrink behavior
│   └── display.rs   # Rendering with Macroquad (grid, snake, apples, game over)
├── Cargo.toml       # Rust package manifest and dependencies
├── Makefile         # Build/run/clean shortcuts
└── README.md
```

## Requirements

- Rust (stable toolchain)
- Cargo

## Build and Run

### Launch with Cargo

```bash
cargo run --release
```

### Controls

- Arrow keys: move the snake
- R: restart after game over
- Esc: quit

## Makefile Commands

### Make shortcuts

```bash
make build      # Build in release mode and copy binary as ./snake
make run        # Run with cargo (optional args: make run ARGS="...")
make clean      # Remove Cargo build artifacts
make fclean     # clean + remove local ./snake binary
make re         # Rebuild from scratch
```

## Notes

- The game starts when you press an arrow key.
- The score shown on screen is the snake length.