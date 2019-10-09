import React from 'react';

export default class Game extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      game: null
    };
  }

  componentDidMount() {
    this.gameWorker = new Worker('../workers/game.worker.js', {
      name: 'game',
      type: 'module'
    });

    this.gameWorker.onmessage = event => {
      const { data } = event;
      switch (data.type) {
        case 'UPDATE_STATE': {
          this.setState({
            game: data.payload
          });
          break;
        }
        case 'INITIALIZE': {
          this.props.onBegin({ gameWorker: this.gameWorker });
          break;
        }
      }
    };

    this.gameWorker.postMessage({
      type: 'RESET_GAME'
    });
  }

  componentWillUnmount() {
    this.gameWorker.terminate();
  }

  render() {
    const { game } = this.state;
    const { render } = this.props;

    return game && render({ game, gameWorker: this.gameWorker });
  }
}
