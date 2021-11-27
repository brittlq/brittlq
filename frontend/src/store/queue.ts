import { defineStore } from 'pinia';
import axios from './axios';

export type User = {
  id: string;
  nickname: string;
  time_joined: Date;
};

export type UserQueue = Array<User>;

export type State = {
  /** The list of users in the queue */
  queue: UserQueue;
  /** How many users to pop when popping the queue */
  partySize: number;
  /** The estimated length betwen groups, in seconds */
  partyTime: number;
  /** Are interactions disabled on the queue components? */
  isDisabled: boolean;
  /** Is the queue accepting new users */
  isOpen: boolean;
  /** The group of users most recently popped from the queue */
  currentGroup: Array<User>;
};

export const useQueueStore = defineStore('queue', {
  state: (): State => ({
    queue: [],
    partySize: 4,
    partyTime: 5 * 60, // seconds
    isDisabled: false,
    isOpen: false,
    currentGroup: new Array<User>(),
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
    removeUserFromQueue(user: User) {
      this.queue = this.queue.filter((entry: User) => {
        return entry.nickname !== user.nickname;
      });
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
