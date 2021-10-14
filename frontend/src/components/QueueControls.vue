<template>
  <nav class="flex flex-col">
    <button class="button-dark" @click="next" :disabled="isDisabled">
      Next
    </button>
    <button
      class="button-dark"
      @click="$emit('toggleOpen', $event)"
      v-if="isOpen"
    >
      Close
    </button>
    <button class="button-dark" @click="$emit('toggleOpen', $event)" v-else>
      Open
    </button>
    <a
      :href="`https://id.twitch.tv/oauth2/authorize?client_id=${clientId}&redirect_uri=${redirectUri}&response_type=${responseType}&scope=${scope}&force_verify=${forceVerify}&claims=${claims}`"
      class="button-dark text-center"
    >
      Connect to Twitch
      <font-awesome-icon :icon="['fab', 'twitch']" />
    </a>
    <div class="flex flex-col">
      <span class="font-bold">Queue size</span>
      <div>{{ queueLength }}</div>
    </div>
    <div class="flex flex-col">
      <span class="font-bold">Time remaining</span>
      <div>{{ timeLeftInQueue }} minutes</div>
    </div>
    <div class="flex flex-col">
      <label class="font-bold">Group Size</label>
      <input
        class="border border-gray-900 rounded p-1"
        v-model="popSize"
        placeholder="4"
      />
    </div>
  </nav>
</template>

<script>
const axios = require('axios').default;
export default {
  name: 'QueueControls',
  data() {
    return {
      isDisabled: false,
      popSize: 4,
      clientId: 've3e62dc7m49kd61qhiz4zt6p3sduk',
      redirectUri: 'http://localhost:9081',
      claims: '{"id_token":{"email":null,"email_verified":null }}',
      forceVerify: 'true',
      scope: 'chat:read+chat:edit',
      responseType: 'token',
    };
  },
  computed: {
    timeLeftInQueue() {
      return Math.floor(this.queueLength / this.popSize) * 5;
    },
  },
  methods: {
    async next(event) {
      try {
        if (event) {
          this.isDisabled = true;
          const url = `/queue/pop?count=${this.popSize}`;
          const { data } = await axios.get(url);
          this.$emit('queuePop', data);
        }
      } catch (exc) {
        console.error(exc);
      } finally {
        this.isDisabled = false;
      }
    },
    async toggleOpen(event) {
      if (event) {
        try {
          const { data } = axios.get('/queue/toggle');
          console.log(data);
          this.isOpen = data.isOpen;
          this.$emit('toggleOpen', data);
        } catch (exc) {
          console.error(exc);
        }
      }
    },
  },
  props: {
    queueLength: {
      required: true,
      type: Number,
    },
    isOpen: {
      required: true,
      type: Boolean,
    },
  },
  emits: ['toggleOpen', 'queuePop'],
};
</script>
