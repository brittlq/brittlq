<template>
	<div class="queue">
		<div class="container">
			<button v-on:click="next">Next</button>
			<button v-on:click="toggle_open" v-if="is_open">Close</button>
			<button v-on:click="toggle_open" v-else>Open</button>
			<a href="https://id.twitch.tv/oauth2/authorize?client_id=2cr6kdblkcnl5d5wrwza6ovhfv2l1g&redirect_uri=http://localhost:8080&response_type=token&scope=chat:read+chat:edit&force_verify=true">
				Connect to chat
			</a>
			<h5 class="index-col">ID Name Time</h5>
			<ul id="array-wth-index">
				<li v-for="(user, index) in queue" :key="user.id">
					<QueueEntry :name="user.nickname" :index="index" :time_joined="user.time_joined"/>
				</li>
			</ul>
		</div>
	</div>
</template>

<script>
import QueueEntry from './QueueEntry';

export default {
	name: 'Queue',
	components: {
		QueueEntry,
	},
	props: [
		"is_open",
		"queue",
	],
	created() {
		this.poll(() => new Promise(() => fetch('http://localhost:8080/status').then((response) => {
			return response.json();
		})
		.then((data) => 
		{
			this.queue = data.queue;
			this.is_open = data.is_open;
		})), 4000);
	},
	mounted() {
		var hash_parameters = location.hash.substr(1);
		var result = hash_parameters.split('&').reduce((res, item) => {
			var parts = item.split('=');
			res[parts[0]] = parts[1];
			return res;
		}, {});
		fetch('http://localhost:8080/token', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(result)
		})
		console.log(result);
	},
	methods: {
		poll(promiseFn, time) {
			var sleep = time => new Promise(resolve => setTimeout(resolve, time))
			promiseFn().then(sleep(time).then(() => this.poll(promiseFn, time)));
		},
		toggle_open(event) {
			if (event) {
				fetch('http://localhost:8080/toggle', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
				}).then((response) => {
					return response.json();
				})
				.then((data) => {
					console.log(data);
					this.is_open = data.is_open;
				})
				.catch((err) => {
					console.log(err);
				})
			}
		},
		next(event) {
			if (event) {
				fetch('http://localhost:8080/next?num=4').then((response) => {
					return response.json();
				})
				.then((data) => {
					this.queue = data
				})
				.catch((err) => {
					console.error(err)
				})
			}
		},
		auth(event) {
			if (event) {
				let token = document.location.hash			
				console.log(token)
			}
		}
	},
}
</script>

<style scoped>
h5 {
	margin: 0;
	padding: 0;
}
ul {
	list-style-type: none;
}
button {
}
.index-col {
}
.name-col {
}
.time-col {
}
ul {
	margin: 0;
	padding: 0;
}
li {
	margin: 0;
	padding: 0;
}
</style>
