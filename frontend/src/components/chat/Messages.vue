<template>
  <div
    :class="[
      'transition-width',
      'flex',
      'flex-col overflow-y-scroll',
      'relative',
      'duration-300',
      { closed: !isChatOpen },
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
import {
  Message as ChatMessage,
  useTwitchChatStore,
} from '@/store/twitch-chat';
import { computed, defineComponent } from 'vue';
export default defineComponent({
  components: { Message },
  setup() {
    const twitchChatStore = useTwitchChatStore();
    twitchChatStore.connectToChat();
    const messages = computed(() => twitchChatStore.messages);
    const isChatOpen = computed(() => twitchChatStore.isChatExpanded);
    return {
      messages,
      isChatOpen,
    };
  },
});
</script>

<style scoped>
.closed {
  @apply w-0;
}
</style>
