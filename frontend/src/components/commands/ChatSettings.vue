<template>
  <div class="flex flex-col w-full mx-3">
    <label>Message</label>
    <textarea
      class="border rounded h-32"
      @change="changed"
      :value="message"
    ></textarea>
  </div>
</template>

<script lang="ts">
import { CommandType } from '@/store/commands';
import { defineComponent, PropType } from 'vue';

export default defineComponent({
  props: {
    name: {
      type: String,
      required: true,
    },
    type: {
      type: String as PropType<CommandType>,
      required: true,
    },
    message: {
      type: String,
      required: true,
    },
  },
  emits: ['update:message'],
  setup(props, { emit }) {
    const changed = (event: Event) => {
      emit(
        'update:message',
        props.name,
        'message',
        (event.target as HTMLTextAreaElement).value
      );
    };
    return {
      changed,
    };
  },
});
</script>

<style></style>
