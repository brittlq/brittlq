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
    <ChatMessage
      v-for="message in messages"
      :key="message.id"
      v-bind="message"
    ></ChatMessage>
  </div>
</template>

<script lang="ts">
import ChatMessage from './Message.vue';
import { useTwitchChatStore } from '@/store/twitch-chat';
import { computed, defineComponent } from 'vue';
export default defineComponent({
  name: 'ChatMessages',
  components: { ChatMessage },
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
