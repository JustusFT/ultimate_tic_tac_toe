import React from 'react';
import Cell from './Cell';

export default function LocalBoard({ cells, boardIndex }) {
  return (
    <div>
      {Array.from(cells).map((cell, cellIndex) => (
        <Cell boardIndex={boardIndex} cellIndex={cellIndex} piece={cell} />
      ))}
    </div>
  );
}
