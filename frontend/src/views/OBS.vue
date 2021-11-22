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
      <button type="button" class="button-dark" @click="getSources">
        Get Sources
      </button>
    </div>
    <ul>
      <li v-for="source in sources" :key="`obs-source-${source}`">
        {{ source }}
      </li>
    </ul>
  </section>
</template>

<script>
import { mapState } from 'vuex';
import {
  OBS_CONNECT,
  OBS_SET_PASSWORD,
  OBS_SET_ADDRESS,
  OBS_UPDATE_SOURCES,
} from '../store/obs/mutations';
export default {
  methods: {
    connect() {
      this.$store.dispatch(OBS_CONNECT);
    },
    getSources() {
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
    ...mapState({
      sources: (state) => state.obs.sources,
    }),
  },
};
</script>

<style></style>
