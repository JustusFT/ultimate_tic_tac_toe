import React, { useState } from 'react';
import styled from 'styled-components';
import Button from './Button';
import HorizontalButtonSet from './HorizontalButtonSet';
import HorizontalSpacer from './HorizontalSpacer';
import Tutorial from './Tutorial';

const VerticalButtonSet = styled.div`
  display: flex;
  flex-direction: column;
  align-items: stretch;
`;

const Spacer = styled.div`
  height: 16px;
`;

export default function Menu({ onResult }) {
  const menuTree = {
    root: (
      <div>
        <h1>Ultimate Tic Tac Toe</h1>
        <VerticalButtonSet>
          <Button block onClick={() => pushHistory('vsCpu')}>
            Play against the CPU
          </Button>
          <Spacer />
          <Button
            block
            onClick={() =>
              onResult({
                type: 'localTwoPlayer'
              })
            }
          >
            Play 2 player game locally
          </Button>
          <Spacer />
          <Button block onClick={() => pushHistory('howToPlay')}>
            How to play
          </Button>
          <Spacer />
          <Button block onClick={() => pushHistory('credits')}>
            Credits
          </Button>
        </VerticalButtonSet>
      </div>
    ),
    vsCpu: (
      <div>
        <p>Which piece will you play?</p>
        <HorizontalButtonSet>
          <Button
            onClick={() =>
              onResult({
                type: 'vsCpu',
                playerPiece: 'X'
              })
            }
          >
            Play X
          </Button>
          <HorizontalSpacer />
          <Button
            onClick={() =>
              onResult({
                type: 'vsCpu',
                playerPiece: 'O'
              })
            }
          >
            Play O
          </Button>
        </HorizontalButtonSet>
      </div>
    ),
    howToPlay: <Tutorial />,
    credits: <div>TODO</div>
  };

  function pushHistory(item) {
    setHistory([...history, item]);
  }

  function popHistory() {
    setHistory(history.slice(0, -1));
  }

  const [history, setHistory] = useState(['root']);
  const currentItem = history[history.length - 1];

  return (
    <div>
      {menuTree[currentItem]}
      <Spacer />
      {history.length > 1 && (
        <Button onClick={() => popHistory()}>Back to menu</Button>
      )}
    </div>
  );
}
