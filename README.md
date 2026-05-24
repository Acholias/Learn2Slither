# Learn2Slither

[English version](README.en.md)

Learn2Slither est un jeu Snake en Rust (GUI Macroquad) qui peut se jouer au clavier **ou** être piloté par une IA en **apprentissage par renforcement** (Q-learning) basée sur une **Q-table**.

## Idée du projet (simple)

Le snake observe un état très compact (danger autour de lui + présence d’une pomme verte dans une direction), choisit une action (haut/droite/bas/gauche), reçoit une récompense, et met à jour sa Q-table pour augmenter ses chances de survivre et de manger des pommes vertes.

## Les 2 principes fondamentaux

### 1) La Q-table

Une **Q-table** est un tableau qui stocke “à quel point c’est une bonne idée” de faire l’action $a$ dans l’état $s$ : $Q(s, a)$.

Dans ce projet :

- **État `State`** = 8 booléens :
	- danger en `Up/Down/Left/Right` (mur ou corps)
	- pomme verte visible en `Up/Down/Left/Right`
- Donc $2^8 = 256$ états possibles (indexés en bitmask).
- **Actions `Action`** = 4 mouvements (Up/Right/Down/Left).

Au final, la Q-table est de taille **256 × 4**.

### 2) Le Q-learning

À chaque pas :

1. On est dans un état $s$
2. On choisit une action $a$ (souvent via **epsilon-greedy**)
3. On obtient une récompense $r$ et un nouvel état $s'$
4. On met à jour $Q(s,a)$

Formule (version vulgarisée mais correcte) :

$$
Q(s,a) \leftarrow Q(s,a) + \alpha \Big( \underbrace{r + \gamma \max_{a'} Q(s',a')}_{\text{la “cible”}} - Q(s,a) \Big)
$$

- $\alpha$ : taux d’apprentissage (à quel point on corrige vite)
- $\gamma$ : importance du futur (récompenses à long terme)
- $\epsilon$ : exploration (on tente parfois au hasard au lieu du “meilleur” coup)

## Récompenses

Le jeu renvoie une récompense de base :

- pomme verte : +2
- pomme rouge : -3
- déplacement “neutre” : -0.05
- game over : -10

Et un petit bonus/malus de shaping : si le snake se rapproche de la pomme verte la plus proche (distance de Manhattan), +0.2, sinon -0.2.

## Prérequis

- Rust (toolchain stable)
- Cargo

## Lancer le projet

Le Makefile fournit les commandes usuelles. Le binaire s’appelle `snake` (voir [Cargo.toml](Cargo.toml)).

### Avec le Makefile (recommandé)

- Build release + copie du binaire en `./snake` :

```bash
make
```

- Entraîner et sauvegarder un modèle (sans GUI) :

```bash
./snake --mode train -sessions 50000 --model models/agent.json"
```

- Charger un modèle et afficher la GUI en mode IA :

```bash
./snake --mode predict --model models/agent.json --visual
```

- Continuer l’entraînement à partir d’un modèle existant, puis afficher :

```bash
./snake --mode predict-train -sessions 20000 --model models/agent.json --visual
```
### Options CLI (résumé)

- `--mode <train|predict|predict-train>`
- `-n, --sessions <u32>` : nombre d’épisodes d’entraînement
- `-m, --model <path>` : chemin du modèle JSON (chargement/sauvegarde)
- `--visual` : ouvre la fenêtre Macroquad
- `--dontlearn` : en GUI, force un comportement greedy (pas d’exploration)
- `--step` : en GUI + IA, avance d’un pas à chaque touche `N`
- `--board-size <4..10>` : taille du plateau

Note : la taille de fenêtre est déduite des arguments au démarrage Macroquad ; pour éviter les surprises, mets `--board-size` avant d’autres options numériques (ex: `--sessions`).

## Contrôles (GUI)

- `ENTER` : start / pause
- `TAB` : activer/désactiver l’IA (avant de start)
- `FLÈCHES` : start + déplacer (mode joueur)
- `R` : relancer après un game over (mode joueur)
- `KP+` : vitesse IA max
- `KP-` : vitesse IA min
- `BACKSPACE` : vitesse joueur
- `SPACE` : afficher la “vision” (raycasts texte)
- `D` : afficher/cacher les logs
- `H` : afficher/cacher l’aide
- `ESC` : quitter

## Architecture (dossier `src/`)

```text
learn2slither/
├── src/
│   ├── main.rs           # Parse CLI, construit/charge l’agent, lance la GUI
│   ├── cli.rs            # Définition des options (clap) et des modes
│   ├── agent.rs          # Q-table + epsilon-greedy + update Q-learning + save/load JSON
│   ├── agent_builder.rs  # Logique “mode” (train/predict/predict-train) + I/O modèles
│   ├── train.rs          # Boucle d’entraînement (épisodes) via Env.step + Agent.update
│   ├── env.rs            # Environnement RL : reset/step + reward shaping distance
│   ├── state.rs          # Encodage d’état (8 bools -> index 0..255) + vision debug
│   ├── action.rs         # Actions discrètes (Up/Right/Down/Left) -> Direction
│   ├── rewards.rs        # Barème de récompenses (constants + compute_reward)
│   ├── board.rs          # Règles du jeu : collisions, spawn pommes, logique d’un tick
│   ├── snake.rs          # Modèle du snake : corps, mouvement, croissance/réduction
│   ├── runtime.rs        # État runtime GUI (pause, vitesse, toggles, board courant)
│   ├── game_loop.rs      # Boucle Macroquad : tick IA/joueur, fin d’épisode, stats
│   ├── input.rs          # Gestion des touches (vitesse, pause, debug, toggle IA)
│   ├── hud.rs            # Panneau HUD (stats, aide, overlay vision)
│   ├── stats.rs          # Agrégation : épisodes, best length, moyenne
│   ├── logger.rs         # Logs conditionnels + codes ANSI
│   └── display.rs        # Rendu du board (grille, snake, pommes) + game over
├── Cargo.toml            # Dépendances (macroquad, clap, serde, rand)
├── Makefile              # Raccourcis build/run/clean (+ purge models)
└── README.md
```

## Commandes Makefile

```bash
make build      # Build en release et copie du binaire en ./snake
make clean      # Supprime les artefacts Cargo
make fclean     # clean + supprime le binaire local ./snake
make purge      # fclean + supprime ./models
make re         # Rebuild complet
```
