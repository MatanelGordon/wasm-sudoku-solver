<script setup lang="ts">
	import Grid from '@/components/GridLayout.vue';
	import { Mat } from '@/types';
	import { useGridStore } from '@/stores/grid';
	const grid = useGridStore();
	const data = [
		[0, 0, 0, 8, 0, 0, 6, 1, 0],
		[2, 1, 0, 0, 0, 0, 0, 5, 0],
		[0, 0, 4, 0, 0, 0, 0, 0, 7],
		[0, 0, 0, 9, 0, 0, 0, 8, 0],
		[0, 0, 0, 1, 6, 0, 0, 0, 0],
		[1, 0, 7, 0, 0, 8, 0, 3, 0],
		[0, 0, 0, 3, 0, 0, 0, 0, 0],
		[0, 0, 9, 0, 0, 0, 0, 0, 4],
		[0, 7, 0, 5, 0, 9, 2, 0, 1]
	];

	const small_data: Mat<number> = [
		[0, 0, 2, 0],
		[2, 0, 0, 0],
		[0, 0, 0, 4],
		[0, 3, 0, 0]
	];

	setTimeout(() => {
		grid.load(data);
	}, 1000);
</script>

<template>
	<main class="main-layout">
		<Grid border-color="var(--color-secondary)" />
	</main>
</template>

<style>
	.main-layout {
		height: 100%;
		width: 100%;
		display: grid;
		place-items: center;
	}
</style>
