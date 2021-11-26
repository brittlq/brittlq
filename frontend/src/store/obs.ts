import ObsWebSocket, { Scene } from 'obs-websocket-js';
import { defineStore } from 'pinia';
import logging from '../utils/logging';

export type State = {
  address: string;
  password: string;
  connection: ObsWebSocket | null;
  sources: Source[];
  scenes: Scene[];
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

export const useObsStore = defineStore('obs', {
  state: (): State => ({
    address: 'localhost:4444',
    password: '',
    connection: null,
    sources: [],
    scenes: [],
    connected: false,
  }),
  actions: {
    async connect(): Promise<ObsWebSocket> {
      try {
        const connection = new ObsWebSocket();
        await connection.connect({
          address: this.address,
          password: this.password,
        });
        connection.on('ConnectionClosed', this.disconnect);
        connection.on('Exiting', this.disconnect);
        this.connection = connection;
        this.connected = true;
        return connection;
      } catch (exc) {
        logging.error(exc);
        throw exc;
      }
    },
    disconnect() {
      this.connection = null;
      this.connected = false;
    },
    async updateSources() {
      try {
        const response = await this.connection?.send('GetSourcesList');
        const sources = response?.sources;
        if (sources) {
          // autoload source settings
          const sourcesWithSettings = [];
          for (const source of sources) {
            const response = await this.connection?.send('GetSourceSettings', {
              sourceName: source.name,
              sourceType: source.type,
            });
            const settings = response?.sourceSettings;
            this.sources.push({
              ...source,
              settings,
            });
          }
        }
      } catch (exc) {
        logging.error(exc);
      }
    },
    async updateScenes() {
      try {
        const resp = await this.connection?.send('GetSceneList');
        this.scenes = resp?.scenes ?? [];
      } catch (exc) {
        logging.error(exc);
      }
    },
  },
});
