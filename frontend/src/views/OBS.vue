<template>
  <section class="flex flex-col mt-4">
    <div class="w-max flex flex-row h-15">
      <label>
        OBS Websocket Password
        <input
          type="password"
          v-model="password"
          class="border rounded p-1 m-1"
        />
      </label>
      <label>
        OBS Websocket Address
        <input type="text" v-model="address" class="border rounded p-1 m-1" />
      </label>
      <button type="button" class="button-dark" @click="connect">
        Connect
      </button>
    </div>
    <div class="flex flex-col">
      <template v-for="(source, index) in sources" :key="`obs_source_${index}`">
        <Source class="flex flex-row" v-bind="source" />
      </template>
    </div>
  </section>
</template>

<script lang="ts">
import { defineComponent } from '@vue/runtime-core';
import { computed } from 'vue';
import Source from '@/components/obs/Source.vue';
import { useObsStore } from '@/store/obs';
import logging from '@/utils/logging';
export default defineComponent({
  components: { Source },
  setup() {
    const obsStore = useObsStore();
    const sources = computed(() => obsStore.sources);
    const address = computed({
      get(): string {
        return obsStore.address;
      },
      set(value: string) {
        obsStore.address = value;
      },
    });
    const password = computed({
      get(): string {
        return obsStore.password;
      },
      set(value: string) {
        obsStore.password = value;
      },
    });
    const connect = async () => {
      try {
        await obsStore.connect();
        await obsStore.updateSources();
      } catch (exc) {
        logging.error(exc);
      }
    };
    return {
      sources,
      address,
      password,
      connect,
    };
  },
});
</script>

<style></style>
