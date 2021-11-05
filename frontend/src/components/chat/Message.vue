<template>
  <div class="flex flex-row">
    <span>{{ username }}:</span>
    <span class="flex-1">{{ msg }}</span>
  </div>
</template>

<script>
export default {
  props: {
    msg: {
      type: String,
      required: true,
    },
    username: {
      type: String,
      required: true,
    },
    userId: {
      type: String,
      required: true,
    },
    isMod: {
      type: Boolean,
      default: false,
    },
    isSub: {
      type: Boolean,
      default: false,
    },
  },
};
</script>

<style></style>
