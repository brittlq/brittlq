<template>
	<div class="QueueEntry-wrapper">
		{{ index }} {{ name }} {{ time_joined }}
	</div>
</template>

<script>
export default {
	name: "QueueEntry",
	props: ["name", "index", "time_joined"],
}
</script>
<style scope>
</style>
