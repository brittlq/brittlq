<template>
  <QueueControls 
  :queue_length="queue.length" 
  @toggle_open="toggle_open" 
  :is_open="is_open" />
  <div class="queue">
    <table class="table table-sm table-hover table-striped">
      <thead>
        <tr>
          <th scope="col">#</th>
          <th scope="col">Name</th>
          <th scope="col">Time</th>
          <th scope="col">Actions</th>
        </tr>
      </thead>
      <tbody>
        <transition-group name="user-table">
          <QueueEntry
            v-for="(user, index) in queue"
            :key="user.id"
            :entry="user"
            :index="index + 1"
            @remove-user="remove"
            class="queue-item"
          ></QueueEntry>
        </transition-group>
      </tbody>
    </table>
  </div>
</template>

<script>
import QueueEntry from "./QueueEntry.vue";
import QueueControls from "./QueueControls";

const axios = require("axios").default;

export default {
  name: "Queue",
  components: { QueueControls, QueueEntry },
  created() {
    this.poll(
      () =>
        new Promise(() =>
          axios
            .get("/queue")
            .then((response) => {
              return response.data;
            })
            .then((data) => {
              this.queue = data.queue;
              this.is_open = data.is_open;
            })
        ),
      4000
    );
  },
  data() {
    return { is_open: false, queue: [] };
  },
  mounted() {
    var hash_parameters = location.hash.substr(1);
    var result = hash_parameters.split("&").reduce((res, item) => {
      var parts = item.split("=");
      res[parts[0]] = parts[1];
      return res;
    }, {});
    axios
      .post("/queue/token", JSON.stringify(result), {
        headers: { "content-type": "application/json" },
      })
      .then((result) => {
        console.log(result);
    });
  },
  methods: {
    poll(promiseFn, time) {
      var sleep = (time) => new Promise((resolve) => setTimeout(resolve, time));
      promiseFn().then(sleep(time).then(() => this.poll(promiseFn, time)));
    },
        remove(user) {
      if (user) {
        console.log("Removing: ", user);
        var index = this.queue.indexOf(user.nickname);
        if (index >= 0) {
          this.queue.splice(index, 1);
        }
        axios.delete("/queue/" + user.nickname).then((response) => {
          console.log("Confirmed removal of ", response);
        });
      }
    },
    toggle_open(event) {
      if (event) {
        axios
          .get("/queue/toggle")
          .then((response) => {
            return response.data;
          })
          .then((data) => {
            console.log(data);
            this.open = data.open;
          })
          .catch((err) => {
            console.log(err);
          });
      }
    },
    auth(event) {
      if (event) {
        let token = document.location.hash;
        console.log(token);
      }
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
