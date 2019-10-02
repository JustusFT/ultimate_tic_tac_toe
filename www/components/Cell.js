import React, { useContext } from 'react';
import { GameContext } from './Game';

export default function Cell({ piece, boardIndex, cellIndex }) {
  const { makeMove } = useContext(GameContext);

  return (
    <span
      onClick={() => {
        makeMove(boardIndex, cellIndex);
      }}
    >
      {piece}
    </span>
  );
}
