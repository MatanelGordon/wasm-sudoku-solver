<script setup lang="ts">
	import { onMounted } from 'vue';
	import Toast from 'primevue/toast';
	import Grid from '@/components/GridLayout.vue';
	import ActionPanel from '@/components/ActionPanel.vue';
	import { GRID_SIZE_KEY } from '@/constants/localStorage';
	import { DEFAULT_GRID_SIZE } from '@/constants/grid';
	import { useGridStore } from '@/stores/grid';
	import { generateSudokuBoard } from '@/logic';

	const grid = useGridStore();

	// initially loading empty grid
	onMounted(() => {
		const size = parseInt(localStorage.getItem(GRID_SIZE_KEY) ?? '0') || DEFAULT_GRID_SIZE;

		grid.load(generateSudokuBoard(size));
	});
</script>

<template>
	<Toast />
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
