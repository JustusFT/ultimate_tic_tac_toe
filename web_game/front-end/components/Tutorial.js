import React, { useEffect, useState } from 'react';
import useGame from '../hooks/useGame';
import GlobalBoard from './GlobalBoard';

const pages = [
  {
    text: (
      <>
        <p>
          In Ultimate Tic Tac Toe, you play 9 games of traditional Tic Tac Toe
          simultaneously layed out in a 3x3 grid.
        </p>
        <p>
          Just like in traditional Tic Tac Toe, 2 players take turns placing
          their pieces on the boards. The first player uses <b>X</b>, and the
          second player uses <b>O</b>,
        </p>
      </>
    ),
    fen:
      '........./........./........./........./........./........./........./........./......... ......... x -'
  },
  {
    text: (
      <>
        <p>
          However, players can't choose which board to play on. That is
          determined by the location of their opponent's last move.
        </p>
        <p>
          For example, lets say X plays at the bottom left board, on the{' '}
          <b>center cell</b>. O must play their next move on the{' '}
          <b>center board</b>.
        </p>
      </>
    ),
    fen:
      '........./........./........./........./........./........./....x..../........./......... ......... o 4'
  },
  {
    text: (
      <p>
        If O plays on the <b>top cell</b>, X must make their next move on the{' '}
        <b>top board</b>, and so on.
      </p>
    ),
    fen:
      '........./........./........./........./.o......./........./....x..../........./......... ......... x 1'
  },
  {
    text: (
      <p>
        If a player manages to get 3 of their pieces in a row, they win that
        board.
      </p>
    ),
    fen:
      'o.....xxx/.ox.xxo.o/o..x.x..o/.oox..x../ox..x.oo./x.xo.o.../.xo..oxo./.x.ox.xo./.o..o..xx x........ o 6'
  },
  {
    text: (
      <>
        <p>
          Once a board is won, no more moves can be made on that board. If a
          player sends their opponent to a won board, then the other player can
          play their next move anywhere else.
        </p>
        <p>
          In this example, X will play on the bottom-left cell, sending O to a
          board that is already won. O can make their next move on any other
          board.
        </p>
      </>
    ),
    fen:
      'o.....xxx/.ox.xxo.o/o..x.x..o/.oox..x../ox..x.oo./x.xo.o.../.xo..oxoo/.x.ox.xo./.o..o..xx x.....o.. x 8'
  },
  {
    text: (
      <p>
        The player who wins 3 boards in a row wins the game. This board shows a
        game won by O.
      </p>
    ),
    fen:
      'x.xoxoxoo/.xxooox../ooo.x..../xx.x.oox./o...o.xxx/xooo.x.xx/ox.oox..o/xo.xo..ox/..x.xxooo xoo.x.ooo x -'
  },
  {
    text: (
      <p>
        If neither player manages to get 3 in a row and no more legal moves can
        be made, then the game is a draw.
      </p>
    ),
    fen:
      'o.....xxx/.ox.xxooo/o..xxx..o/xoox..xx./ox.ox.oo./x.xooo.o./.xo..oxoo/oxooxxxox/.o..o.xxx xoxxooo-x x -'
  },
  {
    text: <p>And that's all you need to know to play Ultimate Tic Tac Toe!</p>,
    fen:
      'o.....xxx/.ox.xxooo/o..xxx..o/xoox..xx./ox.ox.oo./x.xooo.o./.xo..oxoo/oxooxxxox/.o..o.xxx xoxxooo-x x -'
  }
];

export default function Tutorial() {
  const [pageIndex, setPageIndex] = useState(0);
  const { game, gameWorker } = useGame({ initialFen: pages[0].fen });

  useEffect(() => {
    gameWorker.current.postMessage({
      type: 'RESET_GAME',
      payload: {
        fen: pages[pageIndex].fen
      }
    });
  }, [pageIndex]);

  return (
    game && (
      <div>
        <GlobalBoard game={game} />
        <div>{pages[pageIndex].text}</div>
        {pageIndex > 0 && (
          <button onClick={() => setPageIndex(pageIndex - 1)}>Prev</button>
        )}
        {pageIndex < pages.length - 1 && (
          <button onClick={() => setPageIndex(pageIndex + 1)}>Next</button>
        )}
      </div>
    )
  );
}
