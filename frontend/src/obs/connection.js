import store from '../store';
import ObsWebSocket from 'obs-websocket-js';

const connection = new ObsWebSocket();

const connect = () => {
  try {
    connection.connect(store.state.obs);
  } catch (exc) {
    console.error(exc);
  }
};

// Subscribe to state mutations to catch setting either the obs address, password or other props
store.subscribe((mutation, state) => {
  // filter to only reconnect when OBS options are changed
  //TODO: consider if there is another option allowing us to identify the data we are interested in
  if (mutation.type.startsWith('OBS')) {
    // disconnect just in case
    connection.disconnect();
    connection.connect(state.obs);
  }
});

export default connection;
