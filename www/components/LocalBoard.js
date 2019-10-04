import React, { useContext } from 'react';
import styled from 'styled-components';
import generateBorders from '../utils/generateBorders';
import Cell from './Cell';
import { GameContext } from './Game';

const Grid = styled.div`
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  ${props => generateBorders(props.index, '2px solid #666')}
  padding: 8px;
  background-color: ${props => (props.active ? 'lightblue' : 'none')};
`;

export default function LocalBoard({ data, boardIndex }) {
  const { game } = useContext(GameContext);
  const active =
    game.current_board === null || game.current_board === boardIndex;

  return (
    <Grid index={boardIndex} active={active}>
      {data.board.map((cell, cellIndex) => (
        <Cell
          boardIndex={boardIndex}
          cellIndex={cellIndex}
          piece={cell}
          active={active}
        />
      ))}
    </Grid>
  );
}
