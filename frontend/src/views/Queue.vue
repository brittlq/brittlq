<template>
  <div class="flex flex-col gap-2">
    <QueueControls
      :queue-length="queue.length"
      @queuePop="queuePop"
      class="
        flex flex-row
        justify-around
        w-full
        p-2
        border-b border-gray-900
        bg-gray-200
      "
    />
    <table class="queue table-auto flex-1">
      <thead>
        <tr>
          <th class="p-1">#</th>
          <th class="p-1">Name</th>
          <th class="p-1">Time</th>
          <th class="p-1">Actions</th>
        </tr>
      </thead>
      <tbody>
        <QueueEntry
          v-for="(user, index) in queue"
          :key="user.id"
          :entry="user"
          :index="index + 1"
          class="queue-item"
        ></QueueEntry>
      </tbody>
    </table>
  </div>
</template>

<script lang="ts">
import QueueEntry from '@/components/queue/QueueEntry.vue';
import QueueControls from '@/components/queue/QueueControls.vue';
import { computed, defineComponent, onUnmounted } from '@vue/runtime-core';
import logging from '@/utils/logging';
import { useQueueStore } from '@/store/queue';

interface Data {
  intervalId: number;
}

export default defineComponent({
  components: { QueueControls, QueueEntry },
  setup() {
    const intervalId = window.setInterval(() => {
      // poll the queue endpoint for updates here
      logging.log('Poll fired');
    }, 500);
    onUnmounted(() => {
      window.clearInterval(intervalId);
    });
    const queueStore = useQueueStore();
    const queue = computed(() => queueStore.queue);

    const queuePop = () => {
      queueStore.popQueue();
    };
    return {
      queue,
      queuePop,
    };
  },
});
</script>

<style scoped>
.queue {
  text-align: center;
}
.user-table-enter-active,
.user-table-leave-active {
  transition: all 1s;
}
.user-table-enter, .user-table-leave-to /* .user-table-leave-active below version 2.1.8 */ {
  opacity: 0;
  transform: translateX(100px);
}
</style>
