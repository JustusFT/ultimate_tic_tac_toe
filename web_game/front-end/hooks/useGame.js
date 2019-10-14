import { useEffect, useRef, useState } from 'react';

export default function useGame({
  initialFen = null,
  onInitialize = () => {}
} = {}) {
  const [game, setGame] = useState(null);

  const gameWorker = useRef(
    new Worker('../workers/game.worker.js', {
      name: 'game',
      type: 'module'
    })
  );

  function newGame(fen = null) {
    gameWorker.current.postMessage({
      type: 'RESET_GAME',
      payload: {
        fen
      }
    });
  }

  useEffect(() => {
    gameWorker.current.onmessage = event => {
      const { data } = event;
      switch (data.type) {
        case 'UPDATE_STATE': {
          setGame(data.payload);
          break;
        }
        case 'INITIALIZE': {
          newGame(initialFen);
          onInitialize();
          break;
        }
      }
    };
    return () => {
      gameWorker.current.terminate();
    };
  }, []);

  return {
    game,
    gameWorker
  };
}
