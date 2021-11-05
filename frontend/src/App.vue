<template>
  <Navbar />
  <router-view v-slot="{ Component }">
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
</template>

<script>
import Navbar from './components/layout/Navbar.vue';

export default {
  name: 'App',
  components: { Navbar },
};
</script>

<style></style>
