# Learn2Slither

[Version française](README.md)

Learn2Slither is a Snake game written in Rust (Macroquad GUI) that you can play with the keyboard **or** let an AI control using **reinforcement learning** (Q-learning) backed by a **Q-table**.

## Project idea (simple)

The snake observes a compact state (danger around it + whether a green apple is visible in a direction), picks an action (up/right/down/left), receives a reward, and updates its Q-table to increase its chances to survive and to eat green apples.

## The 2 core concepts

### 1) The Q-table

A **Q-table** is a table of values that answers: “how good is it to take action $a$ in state $s$?” i.e. $Q(s, a)$.

In this project:

- **State `State`** = 8 booleans:
	- danger in `Up/Down/Left/Right` (wall or body)
	- green apple visible in `Up/Down/Left/Right`
- So $2^8 = 256$ possible states (bitmask indexed).
- **Actions `Action`** = 4 moves (Up/Right/Down/Left).

So the Q-table size is **256 × 4**.

### 2) Q-learning

At each step:

1. we are in state $s$
2. we choose action $a$ (typically **epsilon-greedy**)
3. we get reward $r$ and the next state $s'$
4. we update $Q(s,a)$

Update rule (friendly but correct):

$$
Q(s,a) \leftarrow Q(s,a) + \alpha \Big( \underbrace{r + \gamma \max_{a'} Q(s',a')}_{\text{the “target”}} - Q(s,a) \Big)
$$

- $\alpha$ : learning rate (how fast we correct)
- $\gamma$ : discount factor (importance of future rewards)
- $\epsilon$ : exploration rate (occasionally try random moves)

## Rewards

The base reward scheme is:

- green apple: +2
- red apple: -3
- neutral move: -0.05
- game over: -10

Plus a small shaping term: if the snake gets closer to the nearest green apple (Manhattan distance), +0.2, otherwise -0.2.

## Requirements

- Rust (stable toolchain)
- Cargo

## Running

The Makefile provides the usual commands. The binary is called `snake` (see [Cargo.toml](Cargo.toml)).

### With the Makefile (recommended)

- Release build + copy the binary to `./snake`:

```bash
make
```

- Train and save a model (no GUI):

```bash
./snake --mode train -sessions 50000 --model models/agent.json"
```

- Load a model and open the GUI in AI mode:

```bash
./snake --mode predict --model models/agent.json --visual
```

- Continue training from an existing model, then display:

```bash
./snake --mode predict-train -n 20000 --model models/agent.json --visual
```

### CLI options (quick)

- `--mode <train|predict|predict-train>`
- `-n, --sessions <u32>`: number of training episodes
- `-m, --model <path>`: JSON model path (load/save)
- `--visual`: opens the Macroquad window
- `--dontlearn`: in GUI, forces greedy behavior (no exploration)
- `--step`: in GUI + AI, advances one step per `N` key press
- `--board-size <4..10>`: board size

Note: the window size is inferred from CLI args at Macroquad startup; to avoid surprises, put `--board-size` before other numeric options (like `--sessions`).

## Controls (GUI)

- `ENTER`: start / pause
- `TAB`: toggle AI (before starting)
- `ARROWS`: start + move (player mode)
- `R`: restart after game over (player mode)
- `KP+`: AI speed max
- `KP-`: AI speed min
- `BACKSPACE`: player speed
- `SPACE`: show “vision” (text rays)
- `D`: show/hide logs
- `H`: show/hide help
- `ESC`: quit

## Architecture (`src/`)

```text
learn2slither/
├── src/
│   ├── main.rs           # Parses CLI, builds/loads the agent, starts the GUI
│   ├── cli.rs            # CLI options (clap) + modes
│   ├── agent.rs          # Q-table + epsilon-greedy + Q-learning update + JSON save/load
│   ├── agent_builder.rs  # Mode logic (train/predict/predict-train) + model I/O
│   ├── train.rs          # Training loop (episodes) via Env.step + Agent.update
│   ├── env.rs            # RL environment: reset/step + distance-based reward shaping
│   ├── state.rs          # State encoding (8 bools -> 0..255) + vision debug helpers
│   ├── action.rs         # Discrete actions (Up/Right/Down/Left) -> Direction
│   ├── rewards.rs        # Reward constants + compute_reward
│   ├── board.rs          # Game rules: collisions, apple spawning, per-tick logic
│   ├── snake.rs          # Snake model: body, movement, growth/shrink
│   ├── runtime.rs        # GUI runtime state (pause, speed, toggles, current board)
│   ├── game_loop.rs      # Macroquad loop: AI/player ticks, episode end, stats
│   ├── input.rs          # Keyboard handling (speed, pause, debug, AI toggle)
│   ├── hud.rs            # HUD panel (stats, help, vision overlay)
│   ├── stats.rs          # Aggregation: episodes, best length, average
│   ├── logger.rs         # Conditional logging + ANSI colors
│   └── display.rs        # Board rendering (grid, snake, apples) + game over overlay
├── Cargo.toml            # Dependencies (macroquad, clap, serde, rand)
├── Makefile              # Build/run/clean shortcuts (+ purge models)
└── README.md
```

## Makefile commands

```bash
make build      # Release build and copy binary to ./snake
make run        # Runs with cargo (optional args: make run ARGS="...")
make clean      # Removes Cargo artifacts
make fclean     # clean + removes local ./snake binary
make purge      # fclean + removes ./models
make re         # Full rebuild
```
