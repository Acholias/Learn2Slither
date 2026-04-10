### Learn2Slither

## Create an IA for playing snake

learn2slither/
├── src/
│   ├── main.rs          # CLI, boucle principale
│   ├── board.rs         # Environnement, plateau, pommes, collisions
│   ├── snake.rs         # Snake, déplacement, longueur
│   ├── state.rs         # Vision du snake (4 directions → State)
│   ├── agent.rs         # Q-table, epsilon-greedy, update
│   ├── rewards.rs       # Définition des récompenses
│   └── display.rs       # Rendu macroquad + TUI terminal
├── models/              # Modèles sauvegardés (.txt ou .json)
├── Cargo.toml
└── README.md
