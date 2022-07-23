<template>
  <div class="flex flex-row">
    <span>{{ username }}:</span>
    <span class="flex-1">{{ msg }}</span>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, PropType } from 'vue';
import { ChatUserstate } from 'tmi.js';

export default defineComponent({
  name: 'ChatMessage',
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
  setup(props) {
    const username = computed(() => props.userstate.username);
    return {
      username,
    };
  },
});
</script>

<style></style>
