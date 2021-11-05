<template>
  <nav>
    <button class="button-dark w-1/6" @click="next" :disabled="isDisabled">
      Next
    </button>
    <button
      class="button-dark w-1/6"
      @click="$emit('toggleOpen', $event)"
      v-text="isOpen ? 'Close' : 'Open'"
    ></button>
    <div class="flex flex-col w-1/6">
      <span class="font-bold">Queue size</span>
      <div>{{ queueLength }}</div>
    </div>
    <div class="flex flex-col w-1/6">
      <span class="font-bold">Time remaining</span>
      <div>{{ timeLeftInQueue }} minutes</div>
    </div>
    <div class="flex flex-col w-1/6">
      <label class="font-bold">Group Size</label>
      <input type="text" class="rounded" v-model="popSize" placeholder="4" />
    </div>
  </nav>
</template>

<script>
export default {
  name: 'QueueControls',
  emits: ['toggleOpen', 'queuePop'],
  props: {
    queueLength: {
      required: true,
      type: Number,
    },
    startOpen: {
      required: true,
      type: Boolean,
    },
  },
  data() {
    return {
      isDisabled: false,
      isOpen: this.startOpen,
      popSize: 4,
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
          const url = `/api/queue/pop?count=${this.popSize}`;
          const { data } = await this.$axios.get(url);
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
          const { data } = await this.$axios.get('/api/queue/toggle');
          console.log(data);
          this.isOpen = data.isOpen;
          this.$emit('toggleOpen', data);
        } catch (exc) {
          console.error(exc);
        }
      }
    },
  },
};
</script>
