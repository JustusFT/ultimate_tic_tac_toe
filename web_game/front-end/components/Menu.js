import React, { useState } from 'react';
import Button from './Button';
import Tutorial from './Tutorial';

export default function Menu({ onResult }) {
  const menuTree = {
    root: (
      <div>
        <Button onClick={() => pushHistory('vsCpu')}>
          Play against the CPU
        </Button>
        <Button
          onClick={() =>
            onResult({
              type: 'localTwoPlayer'
            })
          }
        >
          Play 2 player game locally
        </Button>
        <Button onClick={() => pushHistory('howToPlay')}>How to play</Button>
        <Button onClick={() => pushHistory('credits')}>Credits</Button>
      </div>
    ),
    vsCpu: (
      <div>
        <p>Which piece will you play?</p>
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
      {history.length > 1 && (
        <Button onClick={() => popHistory()}>Go back</Button>
      )}
    </div>
  );
}
