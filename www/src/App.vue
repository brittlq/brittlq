<template>
	<div id="app">
		<Queue v-bind:queue="queue"/>
	</div>
</template>

<script>
import Queue from "./components/Queue"

export default {
	name: 'App',
	components: {
		Queue
	},
	data() {
		return {
			queue: [ ],
		}
	}
}
</script>

<style>
</style>
