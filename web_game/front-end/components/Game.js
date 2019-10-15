import React from 'react';
import useGame from '../hooks/useGame';
import GlobalBoard from './GlobalBoard';

export default function Game({ gameMode, render }) {
  const { game, gameWorker } = useGame({
    onInitialize: () => {
      if (gameMode.type === 'vsCpu' && gameMode.playerPiece === 'O') {
        gameWorker.current.postMessage({
          type: 'CPU_MOVE'
        });
      }
    }
  });

  return game ? (
    render({
      board: (
        <GlobalBoard
          game={game}
          onMove={(boardIndex, cellIndex) => {
            if (
              game.winner ||
              game.local_boards[boardIndex].claimed ||
              game.local_boards[boardIndex].board[cellIndex] !== 'BLANK' ||
              (game.current_board !== null &&
                game.current_board !== boardIndex) ||
              (gameMode.type === 'vsCpu' && game.turn !== gameMode.playerPiece)
            ) {
              return;
            }
            gameWorker.current.postMessage({
              type: 'PLAYER_MOVE',
              payload: [boardIndex, cellIndex]
            });
            if (gameMode.type === 'vsCpu' && !game.winner) {
              gameWorker.current.postMessage({
                type: 'CPU_MOVE'
              });
            }
          }}
        />
      )
    })
  ) : (
    <div>Loading game...</div>
  );
}
