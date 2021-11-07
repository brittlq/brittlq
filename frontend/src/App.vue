<template>
  <div class="flex flex-col h-screen">
    <Navbar />
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
      <Chat class="border-l border-gray-900 transition-width w-1/3" />
    </section>
  </div>
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
