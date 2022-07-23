<template>
  <tr>
    <th scope="row" class="p-1">{{ index }}</th>
    <td class="p-1">{{ entry.nickname }}</td>
    <td class="p-1">{{ entry.time_joined }}</td>
    <td class="p-1">
      <button @click="removeUser(entry)" class="btn btn-outline-danger">
        <fa-icon icon="minus-circle" />
        remove
      </button>
    </td>
  </tr>
</template>

<script lang="ts">
import { useQueueStore, User } from '@/store/queue';
import { defineComponent, PropType } from 'vue';

export default defineComponent({
  props: {
    entry: {
      required: true,
      type: Object as PropType<User>,
    },
    index: {
      required: true,
      type: Number,
    },
  },
  setup() {
    const queueStore = useQueueStore();
    return {
      removeUser: queueStore.removeUserFromQueue,
    };
  },
});
</script>
