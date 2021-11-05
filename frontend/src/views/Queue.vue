<template>
  <div class="flex flex-col gap-2">
    <QueueControls
      :queue-length="queue.length"
      @queuePop="queuePop"
      :start-open="isOpen"
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
          @remove-user="remove"
          class="queue-item"
        ></QueueEntry>
      </tbody>
    </table>
  </div>
</template>

<script>
import QueueEntry from '@/components/queue/QueueEntry.vue';
import QueueControls from '@/components/queue/QueueControls.vue';

const axios = require('axios').default;

export default {
  name: 'Queue',
  components: { QueueControls, QueueEntry },
  data() {
    return { isOpen: false, queue: [], isConnected: false, intervalId: null };
  },
  created() {},
  mounted() {
    //TODO: convert this to a websocket server? Avoids uneccessary network overhead
    // this.intervalId = window.setInterval(this.poll, 4000);
  },
  unmounted() {
    window.clearInterval(this.intervalId);
  },
  methods: {
    async poll() {
      try {
        const { data } = await axios.get('/queue/');
        this.queue = data.queue;
        this.is_open = data.is_open;
      } catch (exc) {
        console.error(exc);
      }
    },
    remove(user) {
      if (user) {
        console.log('Removing: ', user);
        var index = this.queue.indexOf(user.nickname);
        if (index >= 0) {
          this.queue.splice(index, 1);
        }
        axios.delete('/queue/' + user.nickname).then((response) => {
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
};
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
