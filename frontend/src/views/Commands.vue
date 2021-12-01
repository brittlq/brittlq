<template>
  <div class="flex flex-col">
    <h1 class="text-2xl">Command List</h1>
    <div
      class="flex flex-col"
      v-for="command in commands"
      :key="`command_${command.name}`"
    >
      <div class="flex flex-row justify-between">
        <div class="flex flex-col mx-3 w-1/3">
          <label>Command Trigger</label>
          <input
            type="text"
            aria-label="Command Trigger"
            placeholder="Command Name/Trigger e.g. say => !say"
            v-model="command.name"
          />
        </div>
        <div class="flex flex-col mx-3 w-1/3">
          <label>Command Type</label>
          <select class="w-full" v-model="command.type">
            <option
              v-for="(cType, name) in CommandType"
              :key="`command_type_${cType}`"
              :value="cType"
            >
              {{ name }}
            </option>
          </select>
        </div>
        <div class="flex flex-col mx-3 w-1/3">
          <label>Required Role</label>
          <select class="w-full" v-model="command.role">
            <option
              v-for="(rType, name) in Role"
              :key="`command_type_${rType}`"
              :value="rType"
            >
              {{ name }}
            </option>
          </select>
          <p class="text-sm text-gray-700">
            The role is cascading, so a command requiring Subscriber role will
            also work for a Moderator, and the Broadcaster
          </p>
        </div>
      </div>
      <div class="flex flex-row">
        <component
          :is="command.type"
          v-bind="command"
          @update:message="updateCommandProperty"
        ></component>
      </div>
    </div>
    <button class="button-dark" @click="addCommand()">Add Command</button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import {
  CommandType,
  Role,
  CommandTypeSettingsComponents,
  useCommandsStore,
} from '@/store/commands';

export default defineComponent({
  name: 'CommandList',
  components: {
    ...CommandTypeSettingsComponents,
  },
  setup() {
    const commandStore = useCommandsStore();
    const commands = commandStore.commands;
    const addCommand = () => {
      commands.push({
        type: CommandType.Chat,
        message: '',
        name: '',
        role: Role.Broadcaster,
      });
    };
    const updateCommandProperty = (
      property: string,
      commandName: string,
      value: string
    ) => {
      const command = commands.find((command) => command.name === commandName);
      if (command) {
        //@ts-ignore //TODO: figure out why this is erroring (besides string accessing a well defined type)
        command[property] = value;
      }
    };
    return {
      CommandType,
      Role,
      commands,
      addCommand,
      updateCommandProperty,
    };
  },
});
</script>

<style></style>
