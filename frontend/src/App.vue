<template>
  <Navbar />
  <section class="flex flex-row">
    <router-view class="w-2/3" v-slot="{ Component }">
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
    <Chat class="w-1/3 border-l border-gray-900 h-screen" />
  </section>
</template>

<script>
import Navbar from './components/layout/Navbar.vue';
import Chat from './components/chat/Messages.vue';

export default {
  name: 'App',
  components: { Navbar, Chat },
};
</script>

<style></style>
