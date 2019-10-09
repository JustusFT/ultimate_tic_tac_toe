import React from 'react';
import styled from 'styled-components';
import generateBorders from '../utils/generateBorders';
import Piece from './Piece';

const CellContainer = styled.div`
  display: inline-flex;
  align-items: center;
  justify-content: center;
  ${props => generateBorders(props.index, '1px solid #999')}
`;

export default function Cell({ boardIndex, cellIndex, piece, onClick }) {
  return (
    <CellContainer
      index={cellIndex}
      onClick={() => onClick(boardIndex, cellIndex)}
    >
      <Piece piece={piece} />
    </CellContainer>
  );
}
