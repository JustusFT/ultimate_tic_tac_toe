import React, { useState } from 'react';
import styled from 'styled-components';
import githubMark from '../assets/GitHub-Mark-32px.png';
import Button from './Button';
import HorizontalButtonSet from './HorizontalButtonSet';
import HorizontalSpacer from './HorizontalSpacer';
import Spacer from './Spacer';
import Tutorial from './Tutorial';

const PieceMenuContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
`;

const Container = styled.div`
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: stretch;
`;

const Content = styled.div`
  flex: 1;
`;

const VerticalButtonSet = styled.div`
  display: flex;
  flex-direction: column;
  align-items: stretch;
`;

const Footer = styled.div`
  display: flex;
  align-items: center;
`;

const FlexGrow = styled.div`
  flex: 1;
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
          <Spacer />
          <Footer>
            <div>Made by Justus Tumacder</div>
            <FlexGrow />
            <a href="https://github.com/justusft">
              <img src={githubMark} width={32} height={32} />
            </a>
            <div
              style={{
                display: 'inline-block',
                width: '2px',
                height: '2em',
                backgroundColor: '#aaa',
                margin: '0 16px'
              }}
            />
            <a href="https://github.com/JustusFT/ultimate_tic_tac_toe">
              Source Code
            </a>
          </Footer>
        </VerticalButtonSet>
      </div>
    ),
    vsCpu: (
      <PieceMenuContainer>
        <div>Which piece will you play?</div>
        <Spacer />
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
      </PieceMenuContainer>
    ),
    howToPlay: <Tutorial />
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
    <Container>
      <div>
        {history.length > 1 && (
          <>
            <Button onClick={() => popHistory()}>Back to menu</Button>
            <Spacer />
          </>
        )}
      </div>
      <Content>{menuTree[currentItem]}</Content>
    </Container>
  );
}
