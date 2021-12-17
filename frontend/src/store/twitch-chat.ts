import { defineStore } from 'pinia';
import { ChatUserstate, Client } from 'tmi.js';
import { v4 as uuid } from 'uuid';
import { useCommandsStore } from './commands';
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
      messages: [] as Message[],
      client: null as Client | null,
      channels: [process.env.VUE_APP_TWITCH_CHANNEL ?? 'brittleknee'],
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
      client.on('message', this.handleMessage);
      client.connect();
      this.client = client;
    },
    handleMessage(
      channel: string,
      userstate: ChatUserstate,
      msg: string,
      self: boolean
    ) {
      if (self) return;
      const messageData = { channel, msg, userstate, id: uuid() };
      this.messages.push(messageData);

      const commandsStore = useCommandsStore();
      if (msg.startsWith(commandsStore.prefix)) {
        const cmdName = msg.split(' ')[0].replace(commandsStore.prefix, ''); // get the first group of characters by whitespace, remove the prefix character
        const cmd = commandsStore.commands.find(
          (command) => command.name === cmdName
        );
        if (cmd) {
          commandsStore.execute(cmd, messageData);
        }
      }
    },
    sendMessage(channel: string, message: string) {
      this.client?.say(channel, message);
    },
  },
});
