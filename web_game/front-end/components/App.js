import React, { useState } from 'react';
import styled from 'styled-components';
import './App.css';
import Button from './Button';
import Game from './Game';
import Menu from './Menu';

const Container = styled.div`
  display: flex;
  justify-content: center;
`;

export default function App() {
  const [gameMode, setGameMode] = useState(null);
  return (
    <Container>
      {gameMode ? (
        <Game
          gameMode={gameMode}
          render={({ board }) => (
            <div>
              <Button onClick={() => setGameMode(null)}>Leave game</Button>
              {board}
            </div>
          )}
        />
      ) : (
        <Menu
          onResult={result => {
            setGameMode(result);
          }}
        />
      )}
    </Container>
  );
}
