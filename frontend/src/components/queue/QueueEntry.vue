<template>
  <tr>
    <th scope="row" class="p-1">{{ index }}</th>
    <td class="p-1">{{ entry.nickname }}</td>
    <td class="p-1">{{ entry.time_joined }}</td>
    <td class="p-1">
      <button
        @click="$emit('remove-user', entry)"
        class="btn btn-outline-danger"
      >
        <fa-icon icon="minus-circle" />
        remove
      </button>
    </td>
  </tr>
</template>

<script>
export default {
  props: {
    entry: {
      required: true,
      type: Object,
    },
    index: {
      required: true,
      type: Number,
    },
  },
  emits: ['remove-user'],
};
</script>
