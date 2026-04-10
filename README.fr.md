# Learn2Slither

[English version](README.md)

Learn2Slither est un petit jeu Snake ecrit en Rust avec une interface graphique basee sur Macroquad.

## Fonctionnalites

- Plateau 15x15
- Controles clavier en temps reel
- Les pommes vertes font grandir le snake
- Les pommes rouges font retrecir le snake
- Detection des collisions (murs et corps)
- Possibilite de relancer apres un game over

## Architecture

```text
learn2slither/
├── src/
│   ├── main.rs      # Boucle de jeu, gestion des inputs, timing, restart
│   ├── board.rs     # Etat du plateau, spawn des pommes, collisions, logique d'un tour
│   ├── snake.rs     # Modele du snake, deplacement, croissance/reduction
│   └── display.rs   # Rendu Macroquad (grille, snake, pommes, game over)
├── Cargo.toml       # Manifest Rust et dependances
├── Makefile         # Raccourcis de build/run/clean
└── README.md
```

## Prerequis

- Rust (toolchain stable)
- Cargo

## Build et lancement

### Lancer avec Cargo

```bash
cargo run --release
```

### Controles

- Fleches: deplacer le snake
- R: relancer apres un game over
- Esc: quitter

## Commandes Makefile

### Raccourcis Make

```bash
make build      # Build en release et copie du binaire en ./snake
make run        # Lance avec cargo (args possibles: make run ARGS="...")
make clean      # Supprime les artefacts Cargo
make fclean     # clean + supprime le binaire local ./snake
make re         # Rebuild complet
```

## Notes

- La partie commence quand tu appuies sur une fleche.
- Le score affiche a l'ecran correspond a la longueur du snake.