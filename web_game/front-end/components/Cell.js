import React from 'react';
import styled from 'styled-components';
import generateBorders from '../utils/generateBorders';
import Piece from './Piece';

const CellContainer = styled.div`
  padding: 3px;
  ${props => generateBorders(props.index, '1px solid #999')}
`;

const Highlight = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  box-sizing: border-box;
  border: 3px dashed ${props => (props.highlighted ? '#333' : 'transparent')};
`;

export default function Cell({
  boardIndex,
  cellIndex,
  piece,
  onClick,
  highlighted
}) {
  return (
    <CellContainer
      index={cellIndex}
      onClick={() => {
        if (onClick) {
          onClick(boardIndex, cellIndex);
        }
      }}
    >
      <Highlight highlighted={highlighted}>
        <Piece piece={piece} />
      </Highlight>
    </CellContainer>
  );
}
