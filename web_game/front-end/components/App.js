import React, { useState } from 'react';
import styled from 'styled-components';
import './App.css';
import Button from './Button';
import Game from './Game';
import GameStatus from './GameStatus';
import Menu from './Menu';
import Spacer from './Spacer';

const Container = styled.div`
  display: flex;
  justify-content: center;
`;

const View = styled.div`
  width: 100%;

  margin: 16px;
  height: calc(100vh - 32px);

  @media screen and (min-width: 640px) {
    margin: 32px;
    height: calc(100vh - 64px);
  }

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
            render={({ game, board }) => (
              <GameContainer>
                <div>
                  <Button onClick={() => setGameMode(null)}>Leave game</Button>
                </div>
                <Spacer />
                <Content>{board}</Content>
                <Spacer />
                <GameStatus game={game} gameMode={gameMode} />
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
