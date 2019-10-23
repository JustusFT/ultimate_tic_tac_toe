// The game will run on a separate thread to keep the UI running smoothly.

import('../wasm-bindings/pkg').then(pkg => {
  // Sends the current game state to the main thread.
  // Call this function whenever you want to give the latest game data to React
  function updateState(game) {
    postMessage({
      type: 'UPDATE_STATE',
      payload: pkg.get_game_state(game)
    });
  }

  let game;
  let searchTree;

  onmessage = function(event) {
    const { data } = event;
    switch (data.type) {
      case 'RESET_GAME': {
        const { fen } = data.payload;
        // throw out the old game
        if (game) {
          game.free();
        }
        if (searchTree) {
          searchTree.free();
        }
        // create new game
        try {
          game = fen ? pkg.new_from_fen(fen) : pkg.new_game();
          searchTree = pkg.new_mcts();
        } catch (e) {
          console.error(e);
        }
        updateState(game);
        break;
      }
      case 'PLAYER_MOVE': {
        const [a, b] = data.payload;
        game.make_move(a, b);
        updateState(game);
        break;
      }
      case 'CPU_MOVE': {
        pkg.cpu_move(game, searchTree);
        updateState(game);
        break;
      }
    }
  };

  postMessage({
    type: 'INITIALIZE'
  });
});
