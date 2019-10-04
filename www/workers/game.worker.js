// The game will run on a separate thread to keep the UI running smoothly.

// Sends the current game state to the main thread.
// Call this function whenever you want to give the latest game data to React
function updateState(game) {
  postMessage({
    type: 'UPDATE_STATE',
    payload: game.get_state()
  });
}

import('../../pkg').then(({ Game }) => {
  const game = Game.new();

  // send initial state
  updateState(game);

  onmessage = function(event) {
    const { data } = event;
    switch (data.type) {
      case 'PLAYER_MOVE': {
        const [a, b] = data.payload;
        game.make_move(a, b);
        updateState(game);
        break;
      }
      case 'CPU_MOVE': {
        game.cpu_move(5);
        updateState(game);
        break;
      }
    }
  };
});
