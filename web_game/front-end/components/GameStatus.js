import React from 'react';

export default function GameStatus({ gameMode, game }) {
  return (
    <div>
      {gameMode.type === 'vsCpu' && (
        <p>
          {game.turn === gameMode.playerPiece
            ? 'Your turn'
            : 'CPU is thinking...'}
        </p>
      )}
      {gameMode.type === 'localTwoPlayer' && <p>{game.turn} to move</p>}
    </div>
  );
}
