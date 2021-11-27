<template>
  <nav>
    <div class="w-1/6 flex flex-col">
      <button class="button-dark" @click="next" :disabled="isDisabled">
        Next
      </button>
      <button
        class="button-dark"
        @click="toggleOpen"
        v-text="openQueueLabel"
      ></button>
    </div>
    <div class="flex flex-col w-1/6">
      <span class="font-bold">Queue size</span>
      <div>{{ queueLength }}</div>
    </div>
    <div class="flex flex-col w-1/6">
      <span class="font-bold">Time remaining</span>
      <div>{{ queueDuration }} minutes</div>
    </div>
    <div class="flex flex-col w-1/6">
      <label class="font-bold">Group Size</label>
      <input type="text" class="rounded" v-model="partySize" placeholder="4" />
    </div>
    <div class="w-1/6">
      {{ currentGroup.join(', ') }}
    </div>
  </nav>
</template>

<script lang="ts">
import { useQueueStore } from '@/store/queue';
import { defineComponent, computed } from 'vue';
import { storeToRefs } from 'pinia';

export default defineComponent({
  name: 'QueueControls',
  setup() {
    const queueStore = useQueueStore();
    const {
      isOpen,
      isDisabled,
      queueLength,
      partySize,
      currentGroup,
      partyTime,
    } = storeToRefs(queueStore);
    const openQueueLabel = computed(() => (isOpen ? 'Close' : 'Open'));
    const queueDuration = computed(
      () => Math.floor(queueLength.value / partySize.value) * partyTime.value
    );
    return {
      isDisabled,
      partySize,
      queueLength,
      currentGroup,
      openQueueLabel,
      queueDuration,
      next: queueStore.popQueue,
      toggleOpen: queueStore.toggleQueueState,
    };
  },
});
</script>
