<template>
  <div class="flex flex-col w-full">
    <h2>OBS Websocket Settings</h2>
    <div class="flex flex-col">
      <label>Server Address</label>
      <input type="text" v-model="address" />
      <p class="text-sm italic text-gray-900">
        This is generally <strong>localhost</strong>
      </p>
    </div>
    <div class="flex flex-col">
      <label>Server Port</label>
      <input type="number" v-model="port" min="0" max="65535" step="1" />
      <p class="text-sm italic text-gray-900">The default port is 4444</p>
    </div>
    <div class="flex flex-col">
      <label>Server Password</label>
      <input type="password" v-model="password" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useObsStore } from '@/store/obs';
import { storeToRefs } from 'pinia';

export default defineComponent({
  name: 'BQSettings',
  setup() {
    const obsStore = useObsStore();
    const { address, port, password } = storeToRefs(obsStore);
    return {
      address,
      port,
      password,
    };
  },
});
</script>
