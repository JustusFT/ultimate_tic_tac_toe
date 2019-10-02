import React from 'react';
import ReactDOM from 'react-dom';

function App() {
  return <div>Hello world</div>;
}

ReactDOM.render(<App />, document.getElementById('root'));

function range(start, length) {
  return Array.from({ length }, (x, i) => i + start);
}

Promise.all([import('../pkg'), import('../pkg/index_bg.wasm')]).then(values => {
  const [{ Game }, { memory }] = values;

  const game = Game.new();

  // store the pointers to the game boards so we can read directly from wasm memory
  const boardPtrs = range(0, 9).map(x => game.get_board_pointer(x));
  const boardCells = boardPtrs.map(
    ptr => new Uint8Array(memory.buffer, ptr, 9)
  );

  // make some test moves
  console.log(boardCells);
  game.make_move(0, 0);
  console.log(boardCells);
  game.cpu_move(6);
  console.log(boardCells);
});
