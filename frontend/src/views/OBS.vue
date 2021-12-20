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
      <button type="button" class="button-dark" @click="updateSources">
        Get Sources
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
import {
  OBS_CONNECT,
  OBS_SET_PASSWORD,
  OBS_SET_ADDRESS,
  OBS_UPDATE_SOURCES,
} from '../store/obs/operations';
import Source from '@/components/obs/Source.vue';
export default defineComponent({
  name: 'OBSView',
  methods: {
    connect() {
      this.$store.dispatch(OBS_CONNECT);
    },
    updateSources() {
      this.$store.dispatch(OBS_UPDATE_SOURCES);
    },
  },
  computed: {
    password: {
      get() {
        return this.$store.state.obs.password;
      },
      set(value) {
        this.$store.commit(OBS_SET_PASSWORD, value);
      },
    },
    address: {
      get() {
        return this.$store.state.obs.address;
      },
      set(value) {
        this.$store.commit(OBS_SET_ADDRESS, { address: value });
      },
    },
    sources() {
      return this.$store.state.obs.sources;
    },
  },
  components: { Source },
});
</script>

<style></style>
