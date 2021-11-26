import { ChatUserstate, Client } from 'tmi.js';
import { ActionContext } from 'vuex';
import { CONNECT_TO_CHAT, PUSH_MESSAGE, SET_CHAT_CLIENT } from './operations';
import { State as RootState } from '@/store';
import { v4 as uuid } from 'uuid';

export interface Message {
  id: string;
  channel: string;
  msg: string;
  userstate: ChatUserstate;
}

export interface State {
  messages: Message[];
  client: Client | null;
  channels: string[];
}

interface Context extends ActionContext<State, RootState> {}

export default {
  state() {
    return {
      messages: [],
      client: null,
      channels: [],
    };
  },
  mutations: {
    [SET_CHAT_CLIENT](state: State, client: Client) {
      state.client = client;
    },
    [PUSH_MESSAGE](state: State, message: Message) {
      state.messages.push(message);
    },
  },
  actions: {
    [CONNECT_TO_CHAT]({ commit, state, rootState }: Context) {
      const client = new Client({
        connection: { reconnect: true, secure: true },
        channels: state.channels,
        identity: {
          username: rootState.common.botName,
          password: rootState.twitch.token,
        },
        options: {
          clientId: rootState.twitch.clientId,
          skipUpdatingEmotesets: true,
        },
      });
      client.on('connected', (addr: string, port: number) => {
        console.log(`Made connection to twitch chat on ${addr}:${port}`);
      });
      client.on('message', (channel, userstate, msg, self) => {
        if (self) return;
        commit(PUSH_MESSAGE, { channel, msg, userstate, id: uuid() });
      });
      client.connect();
      commit(SET_CHAT_CLIENT, client);
    },
  },
};
