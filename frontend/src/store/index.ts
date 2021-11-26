import { createStore } from 'vuex';
import VuexPersist from 'vuex-persist';
import { TOGGLE_CHAT_SIDEBAR } from './mutations';
import axios from './axios';
import queue, { State as QueueState } from './queue';
import twitch, { State as TwitchState } from './twitch';

export interface CommonState {
  botName: string | undefined;
  channels: string[];
  chatSidebarOpen: boolean;
}

export interface State {
  queue: QueueState;
  twitch: TwitchState;
  common: CommonState;
}

const vuexLocal = new VuexPersist<State>({
  storage: localStorage,
});

const common = {
  state(): CommonState {
    return {
      botName: process.env.VUE_APP_BOT_NAME,
      channels: [process.env.VUE_APP_TWITCH_CHANNEL ?? ''], // the elvis here feels...odd, but can't think of a better solution right now
      chatSidebarOpen: true, //TODO: this should come from application state, either stored in the backend or on the client
    };
  },
  getters: {},
  mutations: {
    [TOGGLE_CHAT_SIDEBAR](state: CommonState) {
      state.chatSidebarOpen = !state.chatSidebarOpen;
    },
  },
};

export { axios };
export default createStore<State>({
  modules: { common, queue, twitch },
  plugins: [vuexLocal.plugin],
});
