import React from 'react';
import styled from 'styled-components';
import generateBorders from '../utils/generateBorders';
import Cell from './Cell';
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
  border: 3px dashed ${props => (props.active ? '#333' : 'transparent')};
  padding: 6px;
`;

const ClaimerOverlay = styled.div`
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
`;

export default function LocalBoard({
  data,
  active,
  boardIndex,
  onMove,
  lastMove
}) {
  return (
    <Container index={boardIndex}>
      <LayerContainer>
        <Layer>
          <Grid active={active}>
            {data.board.map((cell, cellIndex) => (
              <Cell
                key={cellIndex}
                boardIndex={boardIndex}
                cellIndex={cellIndex}
                piece={cell}
                onClick={onMove}
                highlighted={
                  lastMove &&
                  lastMove.board_index === boardIndex &&
                  lastMove.piece_index === cellIndex
                }
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
