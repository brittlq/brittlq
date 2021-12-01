import ObsWebSocket, { Scene, SceneItem } from 'obs-websocket-js';
import { defineStore, StoreActions } from 'pinia';
import logging from '../utils/logging';
import { OBSCommand, OBSCommandActions } from './commands';

export const useObsStore = defineStore('obs', {
  state: () => ({
    address: 'localhost',
    port: 4444,
    password: '',
    connection: null as ObsWebSocket | null,
    scenes: [] as Scene[],
    connected: false,
  }),
  actions: {
    async connect(): Promise<ObsWebSocket> {
      if (this.connected && this.connection) {
        return this.connection;
      } else {
        try {
          const connection = new ObsWebSocket();
          await connection.connect({
            address: `${this.address}:${this.port}`,
            password: this.password,
          });
          connection.on('ConnectionClosed', this.disconnect);
          connection.on('Exiting', this.disconnect);
          connection.on('ConnectionOpened', (data) => {
            logging.log(
              `Connected to OBS websocket server on ${this.address}:${this.port}`
            );
          });
          this.connection = connection;
          this.connected = true;
          return connection;
        } catch (exc) {
          logging.error(exc);
          throw exc;
        }
      }
    },
    disconnect() {
      this.connection = null;
      this.connected = false;
    },
    async reconnect() {
      this.disconnect();
      await this.connect();
    },
    async updateScenes() {
      try {
        const resp = await this.connection?.send('GetSceneList');
        this.scenes = resp?.scenes ?? [];
      } catch (exc) {
        logging.error(exc);
      }
    },
    async toggleSourceVisibility(sourceName: string, sourceId: number) {
      try {
        const sourceProps = await this.connection?.send(
          'GetSceneItemProperties',
          { item: { name: sourceName, id: sourceId } }
        );
        if (sourceProps) {
          sourceProps.visible = !sourceProps.visible;
          const resp = await this.connection?.send('SetSceneItemProperties', {
            item: { name: sourceName, id: sourceId },
            ...sourceProps,
          });
        }
      } catch (exc) {
        logging.error(exc);
      }
    },
    executeCommand(command: OBSCommand) {
      switch (command.action) {
        case OBSCommandActions.ToggleSourceVisibility:
          this.toggleSourceVisibility(command.args.name!, command.args.id!);
          break;
      }
    },
  },
  persist: {
    enabled: true,
    reducer(state) {
      return {
        address: state.address,
        port: state.port,
        password: state.password,
        connected: state.connected,
      };
    },
    hydrater(storedState, context) {
      const newState = JSON.parse(storedState);
      const wasConnected = !!newState.connected;
      delete newState.connected;
      Object.assign(context.store.$state, newState);
      if (wasConnected) {
        context.store.connect();
      }
      return newState;
    },
  },
});
