<template>
  <div>
    <h3 class="w-2/6">{{ name }}</h3>
    <button
      type="button"
      class="w-1/6 border border-gray-900 bg-gray-200 rounded"
      @click="toggleSourceVisibility(name, id)"
    >
      Toggle
    </button>
  </div>
</template>
<script lang="ts">
import { useObsStore } from '@/store/obs';
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
    id: {
      type: Number,
      required: true,
    },
    visible: {
      type: Boolean,
      required: false,
    },
  },
  setup() {
    const obsStore = useObsStore();
    return {
      toggleSourceVisibility: obsStore.toggleSourceVisibility,
    };
  },
});
</script>
