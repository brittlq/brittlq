<template>
  <div
    :class="[
      'transition-width',
      'flex',
      'flex-col overflow-y-scroll',
      'relative',
      { closed: !open },
    ]"
  >
    <message
      v-for="message in messages"
      :key="message.id"
      v-bind="message"
    ></message>
  </div>
</template>

<script>
import Message from './Message.vue';
import { Client } from 'tmi.js';
import { mapState } from 'vuex';
export default {
  components: { Message },
  data() {
    return {
      messages: [],
      client: null,
    };
  },
  mounted() {
    this.client = new Client({
      connection: { reconnect: true, secure: true },
      channels: [this.channel],
      identity: {
        username: this.botName,
        password: this.token,
      },
      options: { clientId: this.clientId, skipUpdatingEmotesets: true },
    });
    this.client.on('connected', (addr, port) => {
      console.log(`Made connection to twitch chat on ${addr}:${port}`);
    });
    this.client.on('message', this.onMessage);
    this.client.connect();
  },
  methods: {
    onMessage(channel, tags, msg, self) {
      if (self) return;
      this.messages.push({
        msg,
        userId: tags['user-id'],
        username: tags.username,
        displayName: tags['display-name'],
        mod: tags.mod,
        sub: tags.subscriber,
      });
    },
  },
  computed: {
    token() {
      return this.$store.state.token;
    },
    botName() {
      return this.$store.state.botName;
    },
    channel() {
      return this.$store.state.channel;
    },
    clientId() {
      return this.$store.state.oauth.twitch.clientId;
    },
    ...mapState({
      open: 'chatSidebarOpen',
    }),
  },
};
</script>

<style scoped>
.closed {
  @apply w-0;
}
</style>
