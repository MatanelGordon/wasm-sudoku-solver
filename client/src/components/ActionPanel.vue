<script setup lang="ts">
	import { onMounted, ref, watch } from 'vue';
	import Button from 'primevue/button';
	import OverlayPanel from 'primevue/overlaypanel';
	import SelectButton from 'primevue/selectbutton';
	import { useToast } from 'primevue/usetoast';
	import type { Mat } from '@/types';
	import { useGridStore } from '@/stores/grid';
	import {
		clone,
		createSquareMatrix,
		generateSudokuBoard,
		isValid,
		map,
		NUMERICAL_TRANSFORMER,
		solve
	} from '@/utils';
	import { controlledWatchEffect, useStorageState } from '@/composables';
	import { DEFAULT_GRID_SIZE, GRID_SIZES } from '@/constants/grid';
	import { GRID_SIZE_KEY } from '@/constants/localStorage';

	const grid = useGridStore();
	const beforeSolve = ref(grid.data);
	const hasPlayed = ref(false);

	//toast
	const toast = useToast();

	// settings related
	const settings_overlay = ref<OverlayPanel | null>(null);
	const GRID_SIZE_OPTIONS = ref(GRID_SIZES.map(value => ({ value, name: `${value}X${value}` })));
	const settings_grid_size = useStorageState<number>(
		GRID_SIZE_KEY,
		DEFAULT_GRID_SIZE,
		NUMERICAL_TRANSFORMER
	);

	watch(
		settings_grid_size,
		() => {
			grid.load(createSquareMatrix(settings_grid_size.value, 0));
		},
		{ immediate: false, flush: 'post' }
	);

	const [setDisableWatch] = controlledWatchEffect(
		grid,
		() => {
			hasPlayed.value = false;
		},
		{
			flush: 'sync'
		}
	);

	function resetGrid(matrix?: Mat<number>) {
		hasPlayed.value = false;
		const data = matrix ?? grid.data;
		const size = data.length;
		grid.load(matrix ?? createSquareMatrix(size, 0));
	}

	async function playHandler() {
		if (hasPlayed.value) {
			const before = map(beforeSolve.value, x => x.value);
			resetGrid(before);
			return;
		}

		if (!isValid(grid.data)) {
			toast.add({
				severity: 'error',
				summary: 'Could not solve invalid board'
			});
			return;
		}

		beforeSolve.value = clone(grid.data);
		const solved = solve(grid.data);
		setDisableWatch(true);
		grid.load(solved);
		hasPlayed.value = true;
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
				:icon="hasPlayed ? pIcon('undo') : pIcon('play')"
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
				@click="settings_overlay?.toggle"
			/>

			<OverlayPanel ref="settings_overlay">
				<SelectButton
					:options="GRID_SIZE_OPTIONS"
					v-model="settings_grid_size"
					option-label="name"
					option-value="value"
				/>
			</OverlayPanel>
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
