<template>
  <nav class="flex flex-col">
    <button class="button-dark" @click="next" :disabled="isDisabled">
      Next
    </button>
    <button
      class="button-dark"
      @click="$emit('toggle_open', $event)"
      v-if="isOpen"
    >
      Close
    </button>
    <button class="button-dark" @click="$emit('toggle_open', $event)" v-else>
      Open
    </button>
    <a
      href='https://id.twitch.tv/oauth2/authorize?client_id=25hshmzbtpompde80gzfr9bkahb9sp&redirect_uri=http://localhost:8080&response_type=token&scope=chat:read+chat:edit&force_verify=true&claims={"id_token":{"email":null,"email_verified":null }}'
      class="button-dark text-center"
    >
      Connect to Twitch
      <font-awesome-icon :icon="['fab', 'twitch']" />
    </a>
    <div><strong>Queue size</strong>{{ queueLength }}</div>
    <div><strong>Time remaining</strong>{{ timeLeftInQueue }} minutes</div>
    <div>
      <label>
        Group Size
        <input
          class="form-control form-control-dark"
          v-model="popSize"
          placeholder="4"
        />
      </label>
    </div>
  </nav>
</template>

<script>
const axios = require('axios').default;
export default {
  name: 'QueueControls',
  data() {
    return { isDisabled: false, popSize: 4 };
  },
  computed: {
    timeLeftInQueue() {
      return Math.floor(this.queueLength / this.popSize) * 5;
    },
  },
  methods: {
    next(event) {
      if (event) {
        let url = `/queue/pop?count=${this.popSize}`;
        axios
          .get(url)
          .then((response) => {
            return response.data;
          })
          .then((data) => {
            this.queue = data;
          })
          .catch((err) => {
            console.error(err);
          });
      }
      this.isDisabled = true;
      var sleep = (time) => new Promise((resolve) => setTimeout(resolve, time));
      sleep(1000).then(() => (this.isDisabled = false));
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
  emits: ['toggle_open'],
};
</script>
