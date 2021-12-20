<template>
  <div
    :class="[
      'transition-width',
      'flex',
      'flex-col overflow-y-scroll',
      'relative',
      'duration-300',
      { closed: !chatOpen },
    ]"
  >
    <Message
      v-for="message in messages"
      :key="message.id"
      v-bind="message"
    ></Message>
  </div>
</template>

<script lang="ts">
import Message from './Message.vue';
import { CONNECT_TO_CHAT } from '@/store/twitch/chat/operations';
import { Message as IMessage } from '@/store/twitch/chat';
import { defineComponent } from 'vue';
export default defineComponent({
  name: 'ChatMessages',
  components: { Message },
  mounted() {
    this.$store.dispatch(CONNECT_TO_CHAT);
  },
  computed: {
    messages(): IMessage[] {
      return this.$store.state.twitch.chat?.messages ?? [];
    },
    chatOpen(): boolean {
      return this.$store.state.common.chatSidebarOpen;
    },
  },
});
</script>

<style scoped>
.closed {
  @apply w-0;
}
</style>
