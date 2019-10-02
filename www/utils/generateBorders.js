// generates css borders for 3x3 tic-tac-toe grid

const directions = ['DR', 'DLR', 'DL', 'UDR', 'UDLR', 'UDL', 'UR', 'ULR', 'UL'];
const directionNames = {
  U: 'top',
  D: 'bottom',
  L: 'left',
  R: 'right'
};

export default function generateBorders(index, borderStyle) {
  return directions[index].split('').reduce((acc, direction) => {
    return acc + `border-${directionNames[direction]}: ${borderStyle};`;
  }, '');
}
