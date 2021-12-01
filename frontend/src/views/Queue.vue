<template>
  <div class="flex flex-col gap-2">
    <queue-controls
      :queue-length="queue.length"
      @queue-pop="queuePop"
      class="flex flex-row justify-around w-full p-2 border-b border-gray-900 bg-gray-200"
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
        <queue-entry
          v-for="(user, index) in queue"
          :key="user.id"
          :entry="user"
          :index="index + 1"
          class="queue-item"
        />
      </tbody>
    </table>
  </div>
</template>

<script lang="ts">
import QueueEntry from '@/components/queue/QueueEntry.vue';
import QueueControls from '@/components/queue/QueueControls.vue';
import { computed, defineComponent, onUnmounted } from '@vue/runtime-core';
import { useQueueStore } from '@/store/queue';

export default defineComponent({
  name: 'PartyQueue',
  components: { QueueControls, QueueEntry },
  setup() {
    const queueStore = useQueueStore();
    // queueStore.startPollingQueue(10000);
    onUnmounted(() => {
      queueStore.stopPollingQueue();
    });
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
