<template>
  <div>
    <h3 class="w-2/6">{{ name }}</h3>
    <button
      type="button"
      class="w-1/6 border border-gray-900 bg-gray-200 rounded"
    >
      Toggle
    </button>
  </div>
</template>
<script lang="ts">
import { defineComponent } from '@vue/runtime-core';

export default defineComponent({
  name: 'OBSSource',
  props: {
    name: {
      type: String,
      required: true,
    },
    type: {
      type: String,
      default: 'unknown',
    },
    typeId: {
      type: String,
      default: 'unknown',
    },
  },
});
</script>
