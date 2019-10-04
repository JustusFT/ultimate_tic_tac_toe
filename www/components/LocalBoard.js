import React, { useContext } from 'react';
import styled from 'styled-components';
import generateBorders from '../utils/generateBorders';
import Cell from './Cell';
import { GameContext } from './Game';
import Piece from './Piece';

const Grid = styled.div`
  height: 100%;
  box-sizing: border-box;
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: repeat(3, 1fr);
  ${props => generateBorders(props.index, '2px solid #666')}
  padding: 8px;
  background-color: ${props => (props.active ? 'lightblue' : 'none')};
`;

const ClaimerOverlay = styled.div`
  height: 100%;
  font-size: 100px;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
`;

const LayerContainer = styled.div`
  height: 100%;
  position: relative;
`;

const Layer = styled.div`
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
`;

export default function LocalBoard({ data, boardIndex }) {
  const { game } = useContext(GameContext);
  const active =
    game.current_board === null || game.current_board === boardIndex;

  const legal = !data.claimer && active;

  return (
    <LayerContainer>
      <Layer>
        <Grid index={boardIndex} active={legal}>
          {data.board.map((cell, cellIndex) => (
            <Cell
              boardIndex={boardIndex}
              cellIndex={cellIndex}
              piece={cell}
              active={legal}
            />
          ))}
        </Grid>
      </Layer>
      {data.claimer && (
        <Layer>
          <ClaimerOverlay>
            <Piece piece={data.claimer} />
          </ClaimerOverlay>
        </Layer>
      )}
    </LayerContainer>
  );
}
