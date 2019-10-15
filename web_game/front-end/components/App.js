import React, { useState } from 'react';
import './App.css';
import Button from './Button';
import Game from './Game';
import Menu from './Menu';

export default function App() {
  const [gameMode, setGameMode] = useState(null);

  return gameMode ? (
    <div>
      <Button onClick={() => setGameMode(null)}>Leave game</Button>
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
