import React from 'react';
import styled from 'styled-components';
import generateBorders from '../utils/generateBorders';

const Container = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
`;

const Grid = styled.span`
  display: inline-grid;
  width: 48px;
  height: 48px;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: repeat(3, 1fr);
`;

const Cell = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
  ${props => generateBorders(props.index, '1px solid #333')}
`;

const CellBox = styled.div`
  @keyframes loading_animation {
    0% {
      width: 80%;
      height: 80%;
    }
    100% {
      width: 0;
      height: 0;
    }
  }
  animation: loading_animation ${props => `${800 / props.speed}ms`} infinite;
  animation-delay: ${props => `${(props.delay * 100) / props.speed}ms`}
  background-color: #333;
`;

const delays = [0, 1, 2, 7, 8, 3, 6, 5, 4];

export default function LoadingIcon() {
  return (
    <Container>
      <Grid>
        {[0, 1, 2, 3, 4, 5, 6, 7, 8].map(x => (
          <Cell index={x} key={x}>
            {x != 4 && <CellBox speed={2} delay={delays[x]} />}
          </Cell>
        ))}
      </Grid>
      <div>Loading...</div>
    </Container>
  );
}
