# Ultimate Tic Tac Toe

A playable game of Ultimate Tic Tac Toe. You can play against an AI or locally with someone else.
The game can be played through your browser here: https://justusft.github.io/ultimate_tic_tac_toe
A terminal implementation of the game is also available. Instructions to run the terminal version can be found [here](#Terminal-Implementation).

## Screenshots

Web implementation:
![Web implementation](/screenshots/web.png?raw=true)

Terminal implementation:
![Terminal implementation](/screenshots/terminal.png?raw=true)

The base game code is written in [Rust](https://www.rust-lang.org/) and compiled to [WebAssembly](https://webassembly.org/) so it can run on the browser. The UI was made with [React](https://reactjs.org/).
The terminal version uses the same base game code but uses [termion](https://github.com/redox-os/termion) for the TUI.
The AI uses [Monte Carlo tree search](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search). UCB1 is used for the selection step. A light playout with random moves is used for the simulation step. A [minimax](https://en.wikipedia.org/wiki/Minimax) implementation is also available, but its not as strong.

## Web Implementation

The game can be played through your browser here: [link]

### Running locally

Open your terminal then run these commands within this repo's root folder.

```
cd web_game
yarn
yarn start
```

The app should start running on localhost:8080

### Building locally

```
cd web_game
yarn build
```

The build should be located at `web_game/front-end/dist`.

## Terminal Implementation

### Running

Open your terminal then run these commands within this repo's root folder.

```
cd terminal_game
cargo run
```

## Testing

Run tests of the base game with

```
cd base_game
cargo test
```

When refactoring the AI, to make sure its behavior doesn't change you can save a consistent simulation of it playing a game.

```
cargo run --bin generate_tests
```

Then when you run the tests it will run another simulation and compare it with the above saved simulation
