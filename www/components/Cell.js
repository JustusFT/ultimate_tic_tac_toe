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

const pieceMap = {
  BLANK: '_',
  X: 'X',
  O: 'O'
};

export default function Cell({ piece, boardIndex, cellIndex, active }) {
  const { makeMove } = useContext(GameContext);

  return (
    <CellContainer
      index={cellIndex}
      onClick={() => {
        // don't allow moves if the board is not active
        if (active) {
          makeMove(boardIndex, cellIndex);
        }
      }}
    >
      {pieceMap[piece]}
    </CellContainer>
  );
}
