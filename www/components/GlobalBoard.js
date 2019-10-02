import React from 'react';
import LocalBoard from './LocalBoard';

export default function GlobalBoard({ localBoards }) {
  return localBoards.map((board, boardIndex) => (
    <LocalBoard boardIndex={boardIndex} cells={board} />
  ));
}
