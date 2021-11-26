import { ActionContext } from 'vuex';
import axios from '../axios';
import { State as RootState } from '..';
import {
  SET_TIME,
  SET_PARTY_SIZE,
  SET_OPEN,
  SET_DISABLED,
  UPDATE,
  REMOVE_USER,
  POP_QUEUE,
  SET_QUEUE,
  TOGGLE_OPEN,
} from './operations';

export interface User {
  id: string;
  nickname: string;
  time_joined: Date;
}

export interface UserQueue extends Array<User> {}

export interface State {
  queue: UserQueue;
  partySize: number;
  partyTime: number;
  isDisabled: boolean;
  isOpen: boolean;
}

interface Context extends ActionContext<State, RootState> {}

export default {
  state: (): State => ({
    queue: [],
    partySize: 4,
    partyTime: 5 * 60,
    isDisabled: false,
    isOpen: false,
  }),
  mutations: {
    [SET_TIME](state: State) {},
    [SET_PARTY_SIZE](state: State, partySize: number) {
      state.partySize = partySize;
    },
    [SET_OPEN](state: State, isOpen: boolean) {
      state.isOpen = isOpen;
    },
    [SET_DISABLED](state: State, disabled: boolean) {
      state.isDisabled = disabled;
    },
    [REMOVE_USER](state: State, { user }: { user: User }) {
      state.queue = state.queue.filter((value) => {
        value.nickname !== user.nickname;
      });
    },
    [SET_QUEUE](state: State, queue: UserQueue) {
      state.queue = queue;
    },
  },
  actions: {
    [UPDATE]({ commit, state }: Context) {},
    async [POP_QUEUE]({ commit, getters }: Context) {
      commit(SET_DISABLED, true);
      const { data } = await axios.get(getters.popUrl);
      commit(SET_QUEUE, data);
    },
    async [TOGGLE_OPEN]({ commit, getters }: Context) {
      const { data } = await axios.get(getters.toggleOpenUrl);
      commit(SET_OPEN, data.isOpen);
    },
  },
  getters: {
    queueLength(state: State) {
      return state.queue.length;
    },
    popUrl(state: State) {
      return `/api/queue/pop?count=${state.partySize}`;
    },
    toggleOpenUrl() {
      return '/api/queue/toggle';
    },
  },
};
