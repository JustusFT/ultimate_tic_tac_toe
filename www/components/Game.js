import React from 'react';
import GlobalBoard from './GlobalBoard';

export const GameContext = React.createContext(null);

export default class Game extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      game: null
    };
  }

  makeMove = (localBoard, cellBoard) => {
    this.gameWorker.postMessage({
      type: 'PLAYER_MOVE',
      payload: [localBoard, cellBoard]
    });

    this.gameWorker.postMessage({
      type: 'CPU_MOVE'
    });
  };

  componentDidMount() {
    this.gameWorker = new Worker('../workers/game.worker.js', {
      name: 'game',
      type: 'module'
    });

    this.gameWorker.onmessage = event => {
      const { data } = event;
      switch (data.type) {
        case 'UPDATE_STATE': {
          this.setState({
            game: data.payload
          });
        }
      }
    };
  }

  render() {
    const { game } = this.state;
    return game ? (
      <GameContext.Provider value={{ game, makeMove: this.makeMove }}>
        {game.turn === 'X' ? 'Your turn' : 'CPU is thinking...'}
        <GlobalBoard localBoards={game.local_boards} />
      </GameContext.Provider>
    ) : (
      <div>Loading...</div>
    );
  }
}
