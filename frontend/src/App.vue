<template>
  <div class="flex flex-col h-screen">
    <navbar />
    <section class="flex-1 flex flex-row">
      <router-view class="flex-1" v-slot="{ Component }">
        <template v-if="Component">
          <transition mode="out-in">
            <keep-alive>
              <suspense>
                <component :is="Component"></component>
                <template #fallback>
                  <div>Loading...</div>
                </template>
              </suspense>
            </keep-alive>
          </transition>
        </template>
      </router-view>
      <chat-messages class="border-l border-gray-900 transition-width w-1/3" />
    </section>
  </div>
</template>

<script lang="ts">
import Navbar from './components/layout/Navbar.vue';
import ChatMessages from './components/chat/Messages.vue';
import { defineComponent } from '@vue/runtime-core';

export default defineComponent({
  name: 'App',
  components: { Navbar, ChatMessages },
});
</script>

<style></style>
