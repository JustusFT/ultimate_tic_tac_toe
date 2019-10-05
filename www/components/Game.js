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
      playerPiece: null,
      type: null
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
    if (this.state.type === 'VS_CPU') {
      this.cpuMove();
    }
  };

  cpuMove = () => {
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
        case 'UPDATE_STATE': {
          this.setState({
            game: data.payload
          });
        }
      }
    };
  }

  render() {
    const { game, playerPiece, type } = this.state;

    if (!game) {
      return <div>Loading...</div>;
    } else if (!type) {
      return (
        <div>
          <button onClick={() => this.setState({ type: 'VS_CPU' })}>
            Play against the CPU
          </button>
          <button
            onClick={() => {
              this.setState({
                type: 'LOCAL_2_PLAYER'
                // playerPiece: 'X'
              });
            }}
          >
            Play local 2 players
          </button>
        </div>
      );
    } else if (type === 'VS_CPU' && !playerPiece) {
      return (
        <div>
          <button onClick={() => this.setState({ playerPiece: 'X' })}>
            Play X
          </button>
          <button
            onClick={() => {
              this.setState({ playerPiece: 'O' });
              this.cpuMove();
            }}
          >
            Play O
          </button>
        </div>
      );
    } else {
      return (
        <GameContext.Provider value={{ game, makeMove: this.makeMove }}>
          <GameContainer>
            <GlobalBoard localBoards={game.local_boards} />
            <Spacer />
            {game.winner ? (
              `${game.winner} won the game`
            ) : (
              <div>
                {type === 'VS_CPU' &&
                  (game.turn === playerPiece
                    ? 'Your turn'
                    : 'CPU is thinking...')}
                {type === 'LOCAL_2_PLAYER' && <div>{game.turn} to play</div>}
                <Spacer />
              </div>
            )}
          </GameContainer>
        </GameContext.Provider>
      );
    }
  }
}
