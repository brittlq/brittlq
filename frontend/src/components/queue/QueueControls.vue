<template>
  <nav>
    <button class="button-dark w-1/6" @click="next" :disabled="isDisabled">
      Next
    </button>
    <button
      class="button-dark w-1/6"
      @click="toggleOpen"
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

<script lang="ts">
import { State } from '@/store';
import {
  POP_QUEUE,
  SET_PARTY_SIZE,
  TOGGLE_OPEN,
} from '@/store/queue/operations';
import { defineComponent } from '@vue/runtime-core';
import { mapState } from 'vuex';

export default defineComponent({
  name: 'QueueControls',
  computed: {
    ...mapState<State>('queue', ['isOpen', 'isDisabled']),
    timeLeftInQueue(): number {
      return Math.floor(this.queueLength / this.popSize) * 5;
    },
    popSize: {
      get(): number {
        return this.$store.state.queue.partySize;
      },
      set(value: number) {
        this.$store.commit(SET_PARTY_SIZE, value);
      },
    },
    queueLength(): number {
      return this.$store.getters['queue/queueLength'];
    },
  },
  methods: {
    async next() {
      try {
        await this.$store.dispatch(`queue/${POP_QUEUE}`);
      } catch (exc) {
        console.error(exc);
      }
    },
    async toggleOpen() {
      try {
        await this.$store.dispatch(`queue/${TOGGLE_OPEN}`);
      } catch (exc) {
        console.error(exc);
      }
    },
  },
});
</script>
