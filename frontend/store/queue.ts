import logging from '@/utils/logging';
import { defineStore } from 'pinia';
import axios from './axios';

export type User = {
  id: string;
  nickname: string;
  time_joined: Date;
};

type QueueResponse = {
  queue: Array<User>;
  is_open: boolean;
};

export const useQueueStore = defineStore('queue', {
  state: () => ({
    queue: new Array<User>(),
    partySize: 4,
    partyTime: 5 * 60, // seconds
    isDisabled: false,
    isOpen: false,
    currentGroup: new Array<User>(),
    pollIntervalId: 0,
  }),
  actions: {
    /**
     * Start listening to the backend for updates to the queue
     * TODO: websocket or some other messaging option?
     */
    startPollingQueue(interval: number = 1000) {
      this.pollIntervalId = window.setInterval(async () => {
        try {
          const { data } = await axios.get<QueueResponse>('/queue');
          this.queue = data.queue;
          this.isOpen = data.is_open;
        } catch (exc) {
          logging.error(exc);
        }
      }, interval);
    },
    stopPollingQueue() {
      window.clearInterval(this.pollIntervalId);
      this.pollIntervalId = 0;
    },
    async popQueue() {
      this.isDisabled = true;
      const { data } = await axios.get(this.popUrl);
      this.queue = data;
      this.isDisabled = false;
    },
    async toggleQueueState() {
      const { data } = await axios.get(this.toggleOpenUrl);
      this.isOpen = data;
    },
    removeUserFromQueue(user: User) {
      this.queue = this.queue.filter((entry: User) => {
        return entry.nickname !== user.nickname;
      });
    },
  },
  getters: {
    queueLength(state): number {
      return state.queue.length;
    },
    popUrl(state): string {
      return `/queue/pop?count=${state.partySize}`;
    },
    toggleOpenUrl(): string {
      return '/queue/toggle';
    },
  },
  persist: {
    enabled: true,
    reducer(state) {
      return {
        partySize: state.partySize,
        partyTime: state.partyTime,
      };
    },
  },
});
