<script setup lang="ts">
	import Button from 'primevue/button';
	import { ref } from 'vue';

	const isPlaying = ref(false);
</script>

<template>
	<div class="action-panel-wrapper">
		<div class="start">
			<Button
				icon="pi pi-play"
				severity="success"
				class="action-button"
				rounded
				:text="!isPlaying"
				@click="isPlaying = !isPlaying"
			/>
			<Button
				icon="pi pi-ban"
				severity="danger"
				class="action-button"
				rounded
				text
			/>
		</div>
		<div class="end">
			<Button
				icon="pi pi-cog"
				class="action-button"
				text
				rounded
			/>
		</div>
	</div>
</template>

<style scoped lang="less">
	.action-panel-wrapper {
		inline-size: 100%;
		margin-bottom: 0.5rem;
		display: flex;
		.start {
			flex: 1;
		}
		.action-button {
			margin-inline: 0.5rem;
		}
	}
</style>
