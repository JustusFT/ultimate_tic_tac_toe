import React, { useContext } from 'react';
import styled from 'styled-components';
import generateBorders from '../utils/generateBorders';
import { GameContext } from './Game';
import Piece from './Piece';

const CellContainer = styled.div`
  display: inline-flex;
  align-items: center;
  justify-content: center;
  ${props => generateBorders(props.index, '1px solid #999')}
`;

export default function Cell({ piece, boardIndex, cellIndex, active }) {
  const { game, makeMove } = useContext(GameContext);

  return (
    <CellContainer
      index={cellIndex}
      onClick={() => {
        // don't allow moves if the board is not active
        if (
          game.local_boards[boardIndex].board[cellIndex] === 'BLANK' &&
          active
        ) {
          makeMove(boardIndex, cellIndex);
        }
      }}
    >
      <Piece piece={piece} />
    </CellContainer>
  );
}
