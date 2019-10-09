import React, { useState } from 'react';
import './App.css';
import Game from './Game';
import GlobalBoard from './GlobalBoard';
import Menu from './Menu';

export default function App() {
  const [gameMode, setGameMode] = useState(null);

  return gameMode ? (
    <div>
      <div onClick={() => setGameMode(null)}>Leave game</div>
      <Game
        onBegin={({ gameWorker }) => {
          if (gameMode.type === 'vsCpu' && gameMode.playerPiece === 'O') {
            gameWorker.postMessage({
              type: 'CPU_MOVE'
            });
          }
        }}
        render={({ game, gameWorker }) => (
          <GlobalBoard
            game={game}
            onMove={(boardIndex, cellIndex) => {
              if (
                game.winner ||
                game.local_boards[boardIndex].claimed ||
                game.local_boards[boardIndex].board[cellIndex] !== 'BLANK' ||
                (game.current_board !== null &&
                  game.current_board !== boardIndex) ||
                (gameMode.type === 'vsCpu' &&
                  game.turn !== gameMode.playerPiece)
              ) {
                return;
              }
              gameWorker.postMessage({
                type: 'PLAYER_MOVE',
                payload: [boardIndex, cellIndex]
              });
              if (gameMode.type === 'vsCpu' && !game.winner) {
                gameWorker.postMessage({
                  type: 'CPU_MOVE'
                });
              }
            }}
          />
        )}
      />
    </div>
  ) : (
    <Menu
      onResult={result => {
        setGameMode(result);
      }}
    />
  );
}
