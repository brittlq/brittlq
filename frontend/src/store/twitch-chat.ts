import { defineStore } from 'pinia';
import { ChatUserstate, Client } from 'tmi.js';
import { v4 as uuid } from 'uuid';
import { useTwitchStore } from './twitch';

export type Message = {
  id: string;
  channel: string;
  msg: string;
  userstate: ChatUserstate;
};

export const useTwitchChatStore = defineStore('twitch/chat', {
  state: () => {
    return {
      messages: new Array<Message>(),
      client: null,
      channels: new Array<string>(),
      isChatExpanded: true,
      botName: process.env.VUE_APP_BOT_NAME,
    };
  },
  actions: {
    connectToChat() {
      const twitchStore = useTwitchStore();
      const client = new Client({
        connection: { reconnect: true, secure: true },
        channels: this.channels,
        identity: {
          username: process.env.VUE_APP_BOT_NAME,
          password: twitchStore.token, //TODO: update once twitch store is updated
        },
        options: {
          clientId: twitchStore.clientId, //TODO: update once twitch store is updated
          skipUpdatingEmotesets: true,
        },
      });
      client.on('connected', (addr: string, port: number) => {
        console.log(`Made connection to twitch chat on ${addr}:${port}`);
      });
      client.on('message', (channel, userstate, msg, self) => {
        if (self) return;
        this.messages.push({ channel, msg, userstate, id: uuid() });
      });
      client.connect();
      this.client = client;
    },
  },
});
