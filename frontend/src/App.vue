<template>
	<div id="app" class="container">
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
			
			queue: [ 
				/* Test array
				{ 
					id: '4efb9fdd-6b1b-468c-9a43-867e3a51c6cc', 
					nickname: 'Abc',
					time_joined: '11:01:32',
					disabled: true,
				},
				{ 
					id: '4e7533f9-28cc-4b4e-a534-2188a6fa1937', 
					nickname: 'Glibbs',
					time_joined: '11:01:32',
					disabled: false,
				},
				{ 
					id: 'a01e576e-06db-4184-b553-93af2b6d6eb8', 
					nickname: 'Joen',
					time_joined: '11:01:32',
					disabled: false,
				},
				*/
			],
		}
	}
}
</script>

<style>
</style>
