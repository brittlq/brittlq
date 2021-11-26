import ObsWebSocket from 'obs-websocket-js';
import {
  OBS_CONNECT,
  OBS_SET_ADDRESS,
  OBS_SET_CONNECTED,
  OBS_SET_CONNECTION,
  OBS_SET_PASSWORD,
  OBS_SET_SOURCES,
  OBS_UPDATE_SOURCES,
} from './operations';
import { ActionContext } from 'vuex';
import { State as RootState } from '..';
import log from '@/utils/logging';

export type State = {
  address: string;
  password: string;
  connection: ObsWebSocket | null;
  sources: Sources;
  connected: boolean;
};

export type SourceSettings = {
  [index: string]: string;
};

export type Source = {
  name: string;
  type: string;
  typeId: string; //TODO: start identifying a list of possible typeIds, create enum or list/type
  settings?: SourceSettings;
};

type Sources = Array<Source>;

export default {
  state: (): State => ({
    address: 'localhost:4444',
    password: '',
    connection: null,
    sources: [],
    connected: false,
  }),
  mutations: {
    [OBS_SET_PASSWORD](state: State, password: string): void {
      state.password = password;
    },
    [OBS_SET_CONNECTION](state: State, connection: ObsWebSocket): void {
      state.connection = connection;
    },
    [OBS_SET_SOURCES](state: State, { sources }: { sources: Sources }): void {
      state.sources = sources;
    },
    [OBS_SET_ADDRESS](state: State, { address }: { address: string }): void {
      state.address = address;
    },
    [OBS_SET_CONNECTED](state: State, connected: boolean): void {
      state.connected = connected;
    },
  },
  actions: {
    async [OBS_CONNECT]({ commit, state }: ActionContext<State, RootState>) {
      try {
        const connection = new ObsWebSocket();
        connection.connect({
          address: state.address,
          password: state.password,
        });
        commit(OBS_SET_CONNECTION, connection);
        commit(OBS_SET_CONNECTED, true);
      } catch (exc) {
        log.error(exc);
      }
    },
    async [OBS_UPDATE_SOURCES]({
      commit,
      state,
    }: ActionContext<State, RootState>) {
      try {
        const response = await state.connection?.send('GetSourcesList');
        const sources = response?.sources;
        if (sources) {
          // autoload source settings
          const sourcesWithSettings = sources.map(async (source) => {
            const response = await state.connection?.send('GetSourceSettings', {
              sourceName: source.name,
              sourceType: source.type,
            });
            const settings = response?.sourceSettings;
            return {
              ...source,
              settings,
            };
          });
          commit(OBS_SET_SOURCES, { sources });
        }
      } catch (exc) {
        console.error(exc);
      }
    },
  },
};
