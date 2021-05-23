<template>
  <tr>
    <th scope="row">{{ index }}</th>
    <td>{{ entry.nickname }}</td>
    <td>{{ entry.time_joined }}</td>
    <td>
      <button
        @click="$emit('remove-user', entry)"
        class="btn btn-outline-danger"
      >
        <font-awesome-icon icon="minus-circle" />
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
  emits: ["remove-user"],
};
</script>
