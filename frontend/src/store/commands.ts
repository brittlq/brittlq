import { defineAsyncComponent } from 'vue';
import { defineStore } from 'pinia';
import { useObsStore } from './obs';
import { Message, useTwitchChatStore } from './twitch-chat';

export enum CommandType {
  Chat = 'ChatSettings',
  OBS = 'OBSSettings',
  Meta = 'MetaSettings',
}

export enum OBSCommandActions {
  ToggleSourceVisibility,
}

export const Role = Object.freeze({
  Broadcaster: 0,
  Moderator: 10,
  Subscriber: 20,
  Follower: 30,
  Viewer: 40,
});

export type RoleKeys = keyof typeof Role;
export type RoleValues = typeof Role[RoleKeys];

export interface BaseCommand {
  id: string;
  name: string;
  type?: CommandType;
  role: RoleValues;
}

export interface ChatCommand extends BaseCommand {
  type?: CommandType.Chat;
  message: string; // should support substitutions
}

export interface OBSCommand extends BaseCommand {
  type?: CommandType.OBS;
  action: OBSCommandActions;
  args: Partial<{
    name: string;
    id: number;
  }>;
  duration?: number;
  running?: boolean;
}

export type Command = ChatCommand | OBSCommand;

export const CommandTypeSettingsComponents = {
  [CommandType.Chat]: defineAsyncComponent(
    () =>
      import(
        /* webpackChunkName: "chat-command-settings" */ '@/components/commands/ChatSettings.vue'
      )
  ),
  [CommandType.OBS]: defineAsyncComponent(
    () =>
      import(
        /* webpackChunkName: "obs-command-settings" */ '@/components/commands/OBSSettings.vue'
      )
  ),
  [CommandType.Meta]: defineAsyncComponent(
    () =>
      import(
        /* webpackChunkName: "meta-command-settings" */ '@/components/commands/MetaSettings.vue'
      )
  ),
};

export const useCommandsStore = defineStore('commands', {
  state: () => ({
    commands: [] as Command[],
    prefix: '!',
  }),
  actions: {
    execute(command: Command, messageData: Message) {
      // do some var substitutions here: ${user} - the user sending the message, ${channel} - the channel where it ocurred
      switch (command.type) {
        case CommandType.Chat: {
          const twitchChat = useTwitchChatStore();
          const newMessage = command.message
            .replaceAll('${user}', messageData.userstate['display-name']!)
            .replaceAll('${channel}', messageData.channel);
          twitchChat.sendMessage(messageData.channel, newMessage);
          break;
        }
        case CommandType.OBS: {
          const obsStore = useObsStore();
          obsStore.executeCommand(command);
          break;
        }
      }
    },
  },
  persist: {
    enabled: true,
  },
});

/**
 * Command types: Chat/Response/Message, OBSCommand, MetaCommand (create/edit/delete commands)
 * Chat/Response/Message: string message, should support substitutions, username of calling user, command params !command param1...
 * OBSCommand: obs actions: Show/Hide Source with optional timer,
 */
