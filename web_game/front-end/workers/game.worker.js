// The game will run on a separate thread to keep the UI running smoothly.

import('../../wasm/pkg').then(pkg => {
  // Sends the current game state to the main thread.
  // Call this function whenever you want to give the latest game data to React
  function updateState(game) {
    console.log(pkg.get_game_state(game));
    postMessage({
      type: 'UPDATE_STATE',
      payload: pkg.get_game_state(game)
    });
  }

  let game;

  try {
    game = pkg.new_from_fen(
      '........./........./....x..../.....o.../....xo.../...x..x../..o....../........./......... x........ o 4'
    );
  } catch (e) {
    console.error(e);
  }

  onmessage = function(event) {
    const { data } = event;
    switch (data.type) {
      case 'RESET_GAME': {
        // throw out the old game and create a new instance
        game.free();
        try {
          game = pkg.new_from_fen(
            '........./........./....x..../.....o.../....xo.../...x..x../..o....../........./......... x........ o 4'
          );
          console.log('asdf2', game.local_boards);
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
        pkg.cpu_move(game, 5);
        updateState(game);
        break;
      }
    }
  };

  postMessage({
    type: 'INITIALIZE'
  });

  // send initial state
  updateState(game);
});
