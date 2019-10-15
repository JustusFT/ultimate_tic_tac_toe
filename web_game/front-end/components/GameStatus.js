import React from 'react';

function getCpuStatusMessage({ gameMode, game }) {
  switch (game.winner) {
    case 'X':
    case 'O': {
      return `${
        gameMode.playerPiece === game.winner ? 'Player' : 'CPU'
      } won the game.`;
    }
    case 'BLANK': {
      return 'The game is a draw.';
    }
    case null: {
      return game.turn === gameMode.playerPiece
        ? 'Your turn'
        : 'CPU is thinking...';
    }
  }
}

function getLocalTwoPlayerStatusMessage({ gameMode, game }) {
  switch (game.winner) {
    case 'X':
    case 'O': {
      return `${game.winner} won the game.`;
    }
    case 'BLANK': {
      return 'The game is a draw.';
    }
    case null: {
      return `${game.turn} to move.`;
    }
  }
}

export default function GameStatus({ gameMode, game }) {
  return (
    <div>
      <p>
        {gameMode.type === 'vsCpu' && getCpuStatusMessage({ gameMode, game })}
        {gameMode.type === 'localTwoPlayer' &&
          getLocalTwoPlayerStatusMessage({ gameMode, game })}
      </p>
    </div>
  );
}
