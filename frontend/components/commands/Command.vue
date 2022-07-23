<template>
  <div class="flex flex-row justify-between">
    <div class="flex flex-col mx-3 w-1/3">
      <label>Command Trigger</label>
      <input
        type="text"
        aria-label="Command Trigger"
        placeholder="Command Name/Trigger e.g. say => !say"
        v-model="name"
      />
    </div>
    <div class="flex flex-col mx-3 w-1/3">
      <label>Command Type</label>
      <select class="w-full" v-model="type">
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
      <select class="w-full" v-model="role">
        <option
          v-for="(rType, name) in Role"
          :key="`command_type_${rType}`"
          :value="rType"
        >
          {{ name }}
        </option>
      </select>
      <p class="text-sm text-gray-700">
        The role is cascading, so a command requiring Subscriber role will also
        work for a Moderator, and the Broadcaster
      </p>
    </div>
  </div>
  <div class="flex flex-row">
    <component
      :is="type"
      v-bind="attrs"
      @update:message="updateCommandProperty"
    ></component>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import {
  CommandType,
  CommandTypeSettingsComponents,
  Role,
  useCommandsStore,
} from '@/store/commands';

export default defineComponent({
  name: 'CommandRow',
  components: {
    ...CommandTypeSettingsComponents,
  },
  props: {
    index: {
      type: Number,
      required: true,
    },
  },
  setup(props, { attrs }) {
    const commandsStore = useCommandsStore();
    const command = commandsStore.commands[props.index];
    const updateCommandProperty = (property: string, value: string) => {
      if (command) {
        //@ts-ignore //TODO: figure out why this is erroring (besides string accessing a well defined type)
        command[property] = value;
      }
    };

    return {
      CommandType,
      Role,
      updateCommandProperty,
      type: ref(command.type),
      name: ref(command.name),
      role: ref(command.role),
      attrs,
    };
  },
});
</script>

<style></style>
