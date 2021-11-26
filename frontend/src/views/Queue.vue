<template>
  <div class="flex flex-col gap-2">
    <QueueControls
      :queue-length="queue.length"
      @queuePop="queuePop"
      class="
        flex flex-row
        justify-around
        w-full
        p-2
        border-b border-gray-900
        bg-gray-200
      "
    />
    <table class="queue table-auto flex-1">
      <thead>
        <tr>
          <th class="p-1">#</th>
          <th class="p-1">Name</th>
          <th class="p-1">Time</th>
          <th class="p-1">Actions</th>
        </tr>
      </thead>
      <tbody>
        <QueueEntry
          v-for="(user, index) in queue"
          :key="user.id"
          :entry="user"
          :index="index + 1"
          class="queue-item"
        ></QueueEntry>
      </tbody>
    </table>
  </div>
</template>

<script lang="ts">
import QueueEntry from '@/components/queue/QueueEntry.vue';
import QueueControls from '@/components/queue/QueueControls.vue';
import { defineComponent } from '@vue/runtime-core';
import { mapState } from 'vuex';
import { UPDATE } from '@/store/queue/operations';

interface Data {
  intervalId: number;
}

export default defineComponent({
  name: 'Queue',
  components: { QueueControls, QueueEntry },
  data(): Data {
    return {
      intervalId: 0,
    };
  },
  created() {},
  mounted() {
    //TODO: convert this to a websocket server? Avoids uneccessary network overhead
    // this.intervalId = window.setInterval(this.poll, 4000);
  },
  unmounted() {
    window.clearInterval(this.intervalId);
  },
  computed: {
    ...mapState('queue', ['queue']),
  },
  methods: {
    async poll() {
      try {
        this.$store.dispatch(UPDATE);
      } catch (exc) {
        console.error(exc);
      }
    },
    remove(user) {
      if (user) {
        console.log('Removing: ', user);
        var index = this.queue.indexOf(user.nickname);
        if (index >= 0) {
          this.$axios.delete('/queue/' + user.nickname).then((response) => {
            this.queue.splice(index, 1);
            console.log('Confirmed removal of ', response);
          });
        }
        this.$axios.delete('/queue/' + user.nickname).then((response) => {
          console.log('Confirmed removal of ', response);
        });
      }
    },
    auth(event) {
      if (event) {
        let token = document.location.hash;
        console.log(token);
      }
    },
    queuePop(queue) {
      this.queue = queue;
    },
  },
});
</script>

<style scoped>
.queue {
  text-align: center;
}
.user-table-enter-active,
.user-table-leave-active {
  transition: all 1s;
}
.user-table-enter, .user-table-leave-to /* .user-table-leave-active below version 2.1.8 */ {
  opacity: 0;
  transform: translateX(100px);
}
</style>
