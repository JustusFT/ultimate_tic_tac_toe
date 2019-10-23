import React from 'react';
import styled from 'styled-components';
import oImg from '../assets/o.svg';
import xImg from '../assets/x.svg';

const pieceMap = {
  X: xImg,
  O: oImg
};

const Img = styled.img`
  width: 90%;
  user-select: none;
`;

export default function Piece({ piece }) {
  return (
    <Img
      onDragStart={e => {
        e.preventDefault();
      }}
      src={pieceMap[piece] || ''}
    />
  );
}
