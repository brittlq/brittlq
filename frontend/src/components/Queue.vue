<template>
  <div class="queue">
    <div>
      <nav class="navbar navbar-dark">
        <button v-on:click="next" :disabled="is_disabled">Next</button>
        <button v-on:click="toggle_open" v-if="is_open">Close</button>
        <button v-on:click="toggle_open" v-else>Open</button>
        <a
          href='https://id.twitch.tv/oauth2/authorize?client_id=25hshmzbtpompde80gzfr9bkahb9sp&redirect_uri=http://localhost:8080&response_type=token&scope=chat:read+chat:edit&force_verify=true&claims={"id_token":{"email":null,"email_verified":null }}'
        >
          Connect to chat
        </a>
        <strong>Queue size</strong>{{ queue.length }}
        <strong>Time remaining</strong
        >{{ Math.floor(queue.length / 4) * party_time }} minutes
        <label for="party_size">Party size</label>
        <input
          class="form-control form-control-dark"
          v-model="pop_size"
          id="party_size"
          placeholder="4"
        />
        <label for="party_time">Minutes per party</label>
        <input
          class="form-control form-control-dark"
          v-model="party_time"
          @change="update_time"
          id="party_time"
          placeholder="5"
        />
      </nav>
    </div>
    <div>
      <table class="table table-sm table-hover table-striped">
        <thead>
          <tr>
            <th scope="col">#</th>
            <th scope="col">Name</th>
            <th scope="col">Time</th>
            <th scope="col">Actions</th>
          </tr>
        </thead>
        <tbody name="user-table" is="transition-group">
          <tr v-for="(user, index) in queue" :key="user.id">
            <th scope="row">{{ index + 1 }}</th>
            <td>{{ user.nickname }}</td>
            <td>{{ user.time_joined }}</td>
            <td>
              <button @click="remove(user)" class="btn btn-outline-danger">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="16"
                  height="16"
                  fill="currentColor"
                  class="bi bi-x-square"
                  viewBox="0 0 16 16"
                >
                  <path
                    d="M14 1a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H2a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1h12zM2 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2H2z"
                  ></path>
                  <path
                    d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"
                  ></path>
                </svg>
                Remove
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script>
export default {
  name: "Queue",
  components: {},
  created() {
    this.is_disabled = false;
    this.poll(
      () =>
        new Promise(() =>
          fetch("http://localhost:8080/queue")
            .then((response) => {
              return response.json();
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
    return { is_disabled: false, pop_size: 4, is_open: false, queue: [], party_time: 5 };
  },
  mounted() {
    var hash_parameters = location.hash.substr(1);
    var result = hash_parameters.split("&").reduce((res, item) => {
      var parts = item.split("=");
      res[parts[0]] = parts[1];
      return res;
    }, {});
    fetch("http://localhost:8080/queue/token", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(result),
    });
    console.log(result);
  },
  computed: {
    incompleteUsers: function () {
      return this.queue.filter(function (user) {
        return !user.disabled;
      });
    },
  },
  methods: {
    remove(user) {
      if (user) {
        console.log("Removing: ", user);
        var index = this.queue.indexOf(user.nickname);
        if (index >= 0) {
          this.queue.splice(index, 1);
        }
        fetch("http://localhost:8080/queue/" + user.nickname, {
          method: "DELETE",
        }).then((response) => {
          console.log("Confirmed removal of ", response);
        });
      }
    },
    update_time(event) {
      if (event) {
        let url = `http://localhost:8080/queue/time?minutes=${this.party_time}`
        fetch(url, {
          method: "PUT"
        })
        .catch((err) => {
          console.error(err);
        });
      }
    },
    poll(promiseFn, time) {
      var sleep = (time) => new Promise((resolve) => setTimeout(resolve, time));
      promiseFn().then(sleep(time).then(() => this.poll(promiseFn, time)));
    },
    toggle_open(event) {
      if (event) {
        fetch("http://localhost:8080/queue/toggle")
          .then((response) => {
            return response.json();
          })
          .then((data) => {
            console.log(data);
            this.is_open = data.is_open;
          })
          .catch((err) => {
            console.log(err);
          });
      }
    },
    next(event) {
      if (event) {
        let url = "http://localhost:8080/queue/pop";
        if (this.pop_size) {
          url = `${url}?count=${this.pop_size}`;
        }
        fetch(url)
          .then((response) => {
            return response.json();
          })
          .then((data) => {
            this.queue = data;
          })
          .catch((err) => {
            console.error(err);
          });
      }
      this.is_disabled = true;
      var sleep = (time) => new Promise((resolve) => setTimeout(resolve, time));
      sleep(1000).then(() => (this.is_disabled = false));
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
