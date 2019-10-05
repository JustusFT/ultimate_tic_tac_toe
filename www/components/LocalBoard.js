import React, { useContext } from 'react';
import styled from 'styled-components';
import generateBorders from '../utils/generateBorders';
import Cell from './Cell';
import { GameContext } from './Game';
import Piece from './Piece';

const Container = styled.div`
  height: 100%;
  box-sizing: border-box;
  ${props => generateBorders(props.index, '2px solid #333')}
`;

const LayerContainer = styled.div`
  height: 100%;
  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 1fr;
`;

const Layer = styled.div`
  grid-column: 1;
  grid-row: 1;
`;

const Grid = styled.div`
  height: calc(100% - 16px);
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: repeat(3, 1fr);
  margin: 8px;
  box-sizing: border-box;
  border: 2px dashed ${props => (props.active ? '#333' : 'transparent')};
  padding: 6px;
`;

const ClaimerOverlay = styled.div`
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
`;

export default function LocalBoard({ data, boardIndex }) {
  const { game } = useContext(GameContext);
  const active =
    !game.winner &&
    (game.current_board === null || game.current_board === boardIndex);

  const legal = !data.claimer && active;

  return (
    <Container index={boardIndex}>
      <LayerContainer>
        <Layer>
          <Grid active={legal}>
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
    </Container>
  );
}
