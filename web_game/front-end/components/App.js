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

const View = styled.div`
  width: 100%;
  height: 100vh;
  min-width: 480px;
  max-width: 960px;
  min-height: 480px;
`;

const Content = styled.div`
  flex: 1;
  overflow: hidden;
`;

const GameContainer = styled.div`
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
`;

export default function App() {
  const [gameMode, setGameMode] = useState(null);

  return (
    <Container>
      <View>
        {gameMode ? (
          <Game
            gameMode={gameMode}
            render={({ board }) => (
              <GameContainer>
                <div>
                  <Button onClick={() => setGameMode(null)}>Leave game</Button>
                </div>
                <Content>{board}</Content>
              </GameContainer>
            )}
          />
        ) : (
          <Menu
            onResult={result => {
              setGameMode(result);
            }}
          />
        )}
      </View>
    </Container>
  );
}
