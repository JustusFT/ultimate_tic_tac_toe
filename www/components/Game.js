import React from 'react';
import GlobalBoard from './GlobalBoard';

function range(start, length) {
  return Array.from({ length }, (x, i) => i + start);
}

export const GameContext = React.createContext(null);

export default class Game extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      initialized: false
    };
    Promise.all([import('../../pkg'), import('../../pkg/index_bg.wasm')]).then(
      values => {
        const [{ Game: GameObj }, { memory }] = values;
        this.game = GameObj.new();
        this.memory = memory;
        // store the pointers to the game boards so we can read directly from wasm memory
        this.boardPtrs = range(0, 9).map(x => this.game.get_board_pointer(x));
        this.updateBoard();
        this.setState({
          initialized: true
        });
      }
    );
  }

  updateBoard = () => {
    this.setState({
      board: this.boardPtrs.map(
        ptr => new Uint8Array(this.memory.buffer, ptr, 9)
      )
    });
  };

  makeMove = (localBoard, cellBoard) => {
    this.game.make_move(localBoard, cellBoard);
    this.updateBoard();
  };

  render() {
    return (
      this.state.initialized && (
        <GameContext.Provider value={{ makeMove: this.makeMove }}>
          <GlobalBoard localBoards={this.state.board} />
        </GameContext.Provider>
      )
    );
  }
}
