import React from 'react';
import styled from 'styled-components';
import LocalBoard from './LocalBoard';

const Grid = styled.div`
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: repeat(3, 1fr);
  width: 600px;
  height: 600px;
`;

export default function GlobalBoard({ game, onMove }) {
  return (
    <Grid>
      {game.local_boards.map((board, boardIndex) => {
        const active =
          game.current_board === null || game.current_board === boardIndex;
        return (
          <LocalBoard
            key={boardIndex}
            data={board}
            active={active}
            boardIndex={boardIndex}
            onMove={onMove}
            lastMove={game.history[game.history.length - 1]}
          />
        );
      })}
    </Grid>
  );
}
