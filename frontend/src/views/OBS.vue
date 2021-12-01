<template>
  <section class="flex flex-col mt-4">
    <ul>
      <li v-for="scene in scenes" :key="scene.name">
        <h3>{{ scene.name }}</h3>
        <ul v-for="source in scene.sources" :key="`obs_source_${source.id}`">
          <Source class="flex flex-row" v-bind="source" />
        </ul>
      </li>
    </ul>
  </section>
</template>

<script lang="ts">
import { defineComponent, watch } from 'vue';
import Source from '@/components/obs/Source.vue';
import { useObsStore } from '@/store/obs';
import { storeToRefs } from 'pinia';
export default defineComponent({
  name: 'OBSControls',
  components: { Source },
  setup() {
    const obsStore = useObsStore();
    const { scenes, connected } = storeToRefs(obsStore);
    obsStore.updateScenes();
    watch(connected, (connected) => {
      if (connected) {
        obsStore.updateScenes();
      }
    });
    return {
      scenes,
    };
  },
});
</script>

<style></style>
