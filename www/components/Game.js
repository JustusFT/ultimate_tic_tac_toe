import React from 'react';
import GlobalBoard from './GlobalBoard';

import styled from 'styled-components';

const GameContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
`;

const Spacer = styled.div`
  height: 16px;
`;

export const GameContext = React.createContext(null);
export default class Game extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      game: null,
      playerPiece: 'X'
    };
  }

  makeMove = (localBoard, cellBoard) => {
    if (this.state.winner) {
      return;
    }
    this.gameWorker.postMessage({
      type: 'PLAYER_MOVE',
      payload: [localBoard, cellBoard]
    });

    // also do cpu move
    if (this.state.winner) {
      return;
    }
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
        case 'WORKER_READY': {
          if (this.state.playerPiece === 'O') {
            this.gameWorker.postMessage({
              type: 'CPU_MOVE'
            });
          }
        }
        case 'UPDATE_STATE': {
          this.setState({
            game: data.payload
          });
        }
      }
    };
  }

  render() {
    const { game, playerPiece } = this.state;

    return game ? (
      <GameContext.Provider value={{ game, makeMove: this.makeMove }}>
        <GameContainer>
          <GlobalBoard localBoards={game.local_boards} />
          <Spacer />
          {game.turn === playerPiece ? 'Your turn' : 'CPU is thinking...'}
          <Spacer />
          {game.winner ? `${game.winner} won the game` : 'Game ongoing'}
        </GameContainer>
      </GameContext.Provider>
    ) : (
      <div>Loading...</div>
    );
  }
}
