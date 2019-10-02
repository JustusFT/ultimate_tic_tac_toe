import React from 'react';
import styled from 'styled-components';
import LocalBoard from './LocalBoard';

const Grid = styled.div`
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  width: 600px;
  height: 600px;
`;

export default function GlobalBoard({ localBoards }) {
  return (
    <Grid>
      {localBoards.map((board, boardIndex) => (
        <LocalBoard boardIndex={boardIndex} cells={board} />
      ))}
    </Grid>
  );
}
