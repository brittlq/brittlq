<template>
  <div class="flex flex-col">
    <h1 class="text-2xl">Command List</h1>
    <div
      class="flex flex-col"
      v-for="(command, index) in commands"
      :key="`command_${command.name}`"
    >
      <CommandRow :index="index" v-bind="command" />
    </div>
    <button class="button-dark" @click="addCommand()">Add Command</button>
  </div>
</template>

<script lang="ts">
import { defineComponent, reactive } from 'vue';
import { Role, useCommandsStore } from '@/store/commands';
import CommandRow from '@/components/commands/Command.vue';
import { v4 } from 'uuid';

export default defineComponent({
  name: 'CommandList',
  components: {
    CommandRow,
  },
  setup() {
    const commandStore = useCommandsStore();
    const commands = commandStore.commands;
    const addCommand = () => {
      const command = reactive({
        id: v4(),
        message: '',
        name: '',
        role: Role.Broadcaster,
        type: undefined,
      });
      commandStore.commands.push(command);
    };

    return {
      commands,
      addCommand,
    };
  },
});
</script>

<style></style>
