Promise.all([import('../pkg'), import('../pkg/index_bg.wasm')]).then(values => {
  const [{ Game }, { memory }] = values;

  const game = Game.new();
  const boardPtr = game.get_board_pointer(0);
  const cells = new Uint8Array(memory.buffer, boardPtr, 9);
  console.log(cells);
});
