import ObsWebSocket from 'obs-websocket-js';
import {
  OBS_CONNECT,
  OBS_SET_ADDRESS,
  OBS_SET_CONNECTION,
  OBS_SET_PASSWORD,
  OBS_SET_SOURCES,
  OBS_UPDATE_SOURCES,
} from './mutations';

export default {
  state: () => ({
    address: 'localhost:4444',
    password: '',
    /** @var {ObsWebSocket} */
    connection: null,
    sources: [],
  }),
  mutations: {
    [OBS_SET_PASSWORD](state, password) {
      state.password = password;
    },
    [OBS_SET_CONNECTION](state, connection) {
      state.connection = connection;
    },
    [OBS_SET_SOURCES](state, { sources }) {
      state.sources = sources;
    },
    [OBS_SET_ADDRESS](state, { address }) {
      state.address = address;
    },
  },
  actions: {
    [OBS_CONNECT]({ commit, state }) {
      const connection = new ObsWebSocket();
      connection.connect({
        address: state.address,
        password: state.password,
      });
      commit(OBS_SET_CONNECTION, connection);
    },
    async [OBS_UPDATE_SOURCES]({ commit, state }) {
      try {
        const sources = await state.connection.send('GetSourcesList');
        commit(OBS_SET_SOURCES, { sources });
      } catch (exc) {
        console.error(exc);
      }
    },
  },
};
