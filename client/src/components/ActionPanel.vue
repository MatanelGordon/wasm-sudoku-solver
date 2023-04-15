<script setup lang="ts">
	import { ref } from 'vue';
	import Button from 'primevue/button';
	import type { Mat } from '@/types';
	import { useGridStore } from '@/stores/grid';
	import { clone, createSquareMatrix, generateSudokuBoard, map, solve } from '@/utils';
	import { controlledWatchEffect } from '@/composables';

	const grid = useGridStore();
	const beforeSolve = ref(grid.data);

	const isPlaying = ref(false);

	const [setDisableWatch] = controlledWatchEffect(
		grid,
		() => {
			isPlaying.value = false;
		},
		{
			flush: 'sync'
		}
	);

	function resetGrid(matrix?: Mat<number>) {
		isPlaying.value = false;
		const data = matrix ?? grid.data;
		const size = data.length;
		grid.load(matrix ?? createSquareMatrix(size, 0));
	}
	function playHandler() {
		if (isPlaying.value) {
			const before = map(beforeSolve.value, x => x.value);
			resetGrid(before);
			return;
		}
		beforeSolve.value = clone(grid.data);
		const solved = solve(grid.data);
		setDisableWatch(true);
		grid.load(solved);
		isPlaying.value = true;
		setDisableWatch(false);
	}

	function generateRandomBoard() {
		const size = grid.size;
		grid.load(generateSudokuBoard(size));
	}

	const pIcon = (name: string) => `pi pi-${name}`;
</script>

<template>
	<div class="action-panel-wrapper">
		<div class="start">
			<Button
				:icon="isPlaying ? pIcon('undo') : pIcon('play')"
				severity="success"
				class="action-button"
				rounded
				text
				@click="playHandler"
			/>
			<Button
				icon="pi pi-ban"
				severity="danger"
				class="action-button"
				@click="resetGrid()"
				rounded
				text
			/>
		</div>
		<div class="end">
			<Button
				icon="pi pi-th-large"
				class="action-button"
				text
				rounded
				@click="generateRandomBoard"
			/>
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
