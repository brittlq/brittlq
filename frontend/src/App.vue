<template>
  <div class="flex flex-row">
    <nav class="mr-10">
      <ul>
        <li>Queue</li>
        <li>Commands</li>
      </ul>
    </nav>
    <Queue />
  </div>
</template>

<script>
import Queue from './components/Queue';

export default {
  name: 'App',
  components: {
    Queue,
  },
};
</script>

<style></style>
