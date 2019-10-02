import React, { useContext } from 'react';
import styled from 'styled-components';
import generateBorders from '../utils/generateBorders';
import { GameContext } from './Game';

const CellContainer = styled.div`
  display: inline-flex;
  align-items: center;
  justify-content: center;
  ${props => generateBorders(props.index, '1px solid #aaa')}
`;

export default function Cell({ piece, boardIndex, cellIndex }) {
  const { makeMove } = useContext(GameContext);

  return (
    <CellContainer
      index={cellIndex}
      onClick={() => {
        makeMove(boardIndex, cellIndex);
      }}
    >
      {piece}
    </CellContainer>
  );
}
