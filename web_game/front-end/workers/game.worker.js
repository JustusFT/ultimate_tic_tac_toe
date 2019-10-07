// The game will run on a separate thread to keep the UI running smoothly.

import('../../wasm/pkg').then(pkg => {
  // Sends the current game state to the main thread.
  // Call this function whenever you want to give the latest game data to React
  function updateState(game) {
    postMessage({
      type: 'UPDATE_STATE',
      payload: pkg.get_game_state(game)
    });
  }

  let game = pkg.new_game();

  onmessage = function(event) {
    const { data } = event;
    switch (data.type) {
      case 'RESET_GAME': {
        // throw out the old game and create a new instance
        game.free();
        game = pkg.new_game();
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
        pkg.cpu_move(game, 5);
        updateState(game);
        break;
      }
    }
  };

  // send initial state
  updateState(game);
});
