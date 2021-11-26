<template>
  <div class="flex flex-row">
    <span>{{ username }}:</span>
    <span class="flex-1">{{ msg }}</span>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from '@vue/runtime-core';
import { ChatUserstate } from 'tmi.js';

export default defineComponent({
  props: {
    msg: {
      type: String,
      required: true,
    },
    userstate: {
      type: Object as PropType<ChatUserstate>,
      required: true,
    },
    id: {
      type: String,
      required: true,
    },
    channel: {
      type: String,
      required: true,
    },
  },
  computed: {
    username(): string {
      return this.userstate.username ?? '';
    },
  },
});
</script>

<style></style>
