import React from 'react';
import styled from 'styled-components';
import generateBorders from '../utils/generateBorders';
import Cell from './Cell';

const Grid = styled.div`
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  ${props => generateBorders(props.index, '2px solid #666')}
  padding: 8px;
`;

export default function LocalBoard({ data, boardIndex }) {
  return (
    <Grid index={boardIndex}>
      {data.board.map((cell, cellIndex) => (
        <Cell boardIndex={boardIndex} cellIndex={cellIndex} piece={cell} />
      ))}
    </Grid>
  );
}
