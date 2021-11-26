import { defineStore } from 'pinia';
import { axios } from '.';

export type User = {
  id: string;
  nickname: string;
  time_joined: Date;
};

export type UserQueue = Array<User>;

export type State = {
  queue: UserQueue;
  partySize: number;
  partyTime: number;
  isDisabled: boolean;
  isOpen: boolean;
};

export const useQueueStore = defineStore('queue', {
  state: (): State => ({
    queue: [],
    partySize: 4,
    partyTime: 5 * 60,
    isDisabled: false,
    isOpen: false,
  }),
  actions: {
    updateQueueFromBackend() {},
    async popQueue() {
      this.isDisabled = true;
      const { data } = await axios.get(this.popUrl);
      this.queue = data;
      this.isDisabled = false;
    },
    async toggleQueueState() {
      const { data } = await axios.get(this.toggleOpenUrl);
      this.isOpen = data.is_open;
    },
  },
  getters: {
    queueLength(state: State): number {
      return state.queue.length;
    },
    popUrl(state: State): string {
      return `/api/queue/pop?count=${state.partySize}`;
    },
    toggleOpenUrl(): string {
      return '/api/queue/toggle';
    },
  },
});
