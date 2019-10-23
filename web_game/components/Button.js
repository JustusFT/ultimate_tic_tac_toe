import styled from 'styled-components';

export default styled.span`
  display: inline-block;
  padding: 8px 16px;
  border: 2px solid #333;
  ${props => (props.block ? 'display: block;' : '')}
`;
