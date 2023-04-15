<script setup lang="ts">
	import Grid from '@/components/GridLayout.vue';
	import ActionPanel from '@/components/ActionPanel.vue';
</script>

<template>
	<main class="main-layout">
		<ActionPanel />
		<Grid border-color="var(--color-secondary)" />
	</main>
</template>

<style>
	.main-layout {
		inline-size: 90vmin;
		display: flex;
		flex-flow: column nowrap;
		justify-content: center;
		align-items: center;
		padding: 0.5rem;
	}
</style>
