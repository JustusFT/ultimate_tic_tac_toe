Promise.all([
  import('ultimate_tic_tac_toe'),
  import('ultimate_tic_tac_toe/ultimate_tic_tac_toe_bg')
]).then(values => {
  const [{ Game }, { memory }] = values;

  const game = Game.new();
  const boardPtr = game.get_board_pointer(0);
  const cells = new Uint8Array(memory.buffer, boardPtr, 9);
  console.log(cells);
});
