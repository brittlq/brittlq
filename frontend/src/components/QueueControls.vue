<template>
  <nav class="navbar navbar-dark">
    <button @click="$emit('next', $event, pop_size)" :disabled="is_disabled">
      Next
    </button>
    <button @click="$emit('toggle_open', $event)" v-if="is_open">Close</button>
    <button @click="$emit('toggle_open', $event)" v-else>Open</button>
    <a
      href='https://id.twitch.tv/oauth2/authorize?client_id=25hshmzbtpompde80gzfr9bkahb9sp&redirect_uri=http://localhost:8080&response_type=token&scope=chat:read+chat:edit&force_verify=true&claims={"id_token":{"email":null,"email_verified":null }}'
    >
      Connect to Twitch
      <font-awesome-icon :icon="['fab', 'twitch']" />
    </a>
    <strong>Queue size</strong>{{ queue_length }} <strong>Time remaining</strong
    >{{ timeLeftInQueue }} minutes
    <input
      class="form-control form-control-dark"
      v-model="pop_size"
      placeholder="4"
    />
  </nav>
</template>

<script>
export default {
  name: "QueueControls",
  data() {
    return { pop_size: 4 };
  },
  computed: {
    timeLeftInQueue() {
      return Math.floor(this.queue_length / this.pop_size) * 5;
    },
  },
  props: {
    queue_length: {
      required: true,
      type: Number,
    },
    is_open: {
      required: true,
      type: Boolean,
    },
    is_disabled: {
      required: true,
      type: Boolean,
    },
  },
  emits: ["next", "toggle_open"],
};
</script>
