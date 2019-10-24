import React, { useEffect, useRef, useState } from 'react';
import Measure from 'react-measure';
import styled from 'styled-components';
import LocalBoard from './LocalBoard';

const Grid = styled.div`
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: repeat(3, 1fr);

  ${props => {
    const minSize = Math.max(
      480,
      Math.min(props.dimensions.width, props.dimensions.height)
    );
    return `
      width: ${minSize}px;
      height: ${minSize}px;
    `;
  }}
`;

const Area = styled.div`
  display: flex;
  justify-content: center;
  width: 100%;
  height: 100%;
`;

export default function GlobalBoard({ game, onMove }) {
  const [dimensions, setDimensions] = useState({ x: -1, y: -1 });
  const redrawRef = useRef(null);

  useEffect(() => {
    // this will force redraw the board
    // it seems like on firefox the board isn't drawn until the user does something like click anywhere on the page
    // the cause seems to be the `mix-blend-mode` style in Cell.js since removing it fixes the issue
    redrawRef.current.style.visibility = 'hidden';
    setTimeout(() => {
      redrawRef.current.style.visibility = 'visible';
    }, 0);
  }, []);

  return (
    <Measure
      bounds
      onResize={contentRect => {
        setDimensions(contentRect.bounds);
      }}
    >
      {({ measureRef }) => (
        <Area ref={measureRef}>
          <div ref={redrawRef}>
            <Grid dimensions={dimensions}>
              {game.local_boards.map((board, boardIndex) => {
                const active =
                  !game.winner &&
                  game.local_boards[boardIndex].claimer === null &&
                  (game.current_board === null ||
                    game.current_board === boardIndex);
                return (
                  <LocalBoard
                    key={boardIndex}
                    data={board}
                    active={active}
                    boardIndex={boardIndex}
                    onMove={onMove}
                    lastMove={game.history[game.history.length - 1]}
                  />
                );
              })}
            </Grid>
          </div>
        </Area>
      )}
    </Measure>
  );
}
