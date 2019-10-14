import React, { useState } from 'react';
import './App.css';
import Game from './Game';
import Menu from './Menu';

export default function App() {
  const [gameMode, setGameMode] = useState(null);

  return gameMode ? (
    <div>
      <div onClick={() => setGameMode(null)}>Leave game</div>
      <Game gameMode={gameMode} />
    </div>
  ) : (
    <Menu
      onResult={result => {
        setGameMode(result);
      }}
    />
  );
}
