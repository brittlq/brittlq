<template>
  <div class="flex flex-row">
    <QueueControls
      :queue-length="queue.length"
      @queuePop="queuePop"
      :start-open="isOpen"
    />
    <table class="queue table-auto">
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
import QueueEntry from './QueueEntry.vue';
import QueueControls from './QueueControls.vue';

const axios = require('axios').default;

export default {
  name: 'Queue',
  components: { QueueControls, QueueEntry },
  data() {
    return { isOpen: false, queue: [], isConnected: false, intervalId: null };
  },
  created() {},
  mounted() {
    var hash_parameters = location.hash.substr(1);
    if (hash_parameters.length > 0) {
      var result = hash_parameters.split('&').reduce((res, item) => {
        var parts = item.split('=');
        res[parts[0]] = parts[1];
        return res;
      }, {});
      axios
        .post('/queue/token', JSON.stringify(result), {
          headers: { 'content-type': 'application/json' },
        })
        .then((result) => {
          console.log(result);
          this.isConnected = true;
        })
        .catch((err) => {
          console.error(err);
        });
    }
    //TODO: convert this to a websocket server? Avoids uneccessary network overhead
    this.intervalId = window.setInterval(this.poll, 4000);
  },
  unmounted() {
    window.clearInterval(this.intervalId);
  },
  methods: {
    async poll() {
      try {
        const { data } = await axios.get('/queue');
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
