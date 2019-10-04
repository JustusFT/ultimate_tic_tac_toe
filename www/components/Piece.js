import React from 'react';
import styled from 'styled-components';
import oImg from '../assets/o.svg';
import xImg from '../assets/x.svg';

const pieceMap = {
  BLANK: '',
  X: xImg,
  O: oImg
};

const Img = styled.img`
  width: 70%;
`;

export default function Piece({ piece }) {
  return <Img src={pieceMap[piece]} />;
}
