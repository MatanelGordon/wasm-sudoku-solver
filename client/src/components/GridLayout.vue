<script setup lang="ts">
	import { computed } from 'vue';
	import { squareTransform } from '@/utils/matrix';
	import GridCell, { CellEventPayload as CellEventPayload } from './GridCell.vue';
	import { useGridStore } from '@/stores/grid';
	import { isMulti } from '@/utils';
	import { useKeydownHandler } from '@/composables';

	const props = defineProps({
		borderColor: {
			type: String,
			default: '#ccc'
		}
	});

	const grid = useGridStore();

	const squares = computed(() => squareTransform(grid.data));

	useKeydownHandler(grid);

	function cellFocus(evt: CellEventPayload<FocusEvent>) {
		const { row, col } = evt;
		grid.setSelected(row, col, true);
	}

	function cellClick(evt: CellEventPayload<MouseEvent>) {
		const shouldNotAddToExisting = !isMulti(evt.event);
		const notSelected = !evt.selected;

		grid.setSelected(evt.row, evt.col, notSelected, notSelected && shouldNotAddToExisting);
	}
</script>

<template>
	<div class="grid">
		<div
			class="square"
			v-for="(square, i) in squares"
			:key="`s-${i}-${square.join('')}`"
		>
			<GridCell
				v-for="(cell, j) in square"
				class="cell"
				:size="grid.size"
				:col="cell.col"
				:row="cell.row"
				:value="cell.value.value"
				:selected="cell.value.selected"
				:key="`c-${i}-${j}-${cell.value}`"
				@focus="cellFocus"
				@click="cellClick"
			/>
		</div>
	</div>
</template>

<style scoped lang="less">
	.square-grid() {
		display: grid;
		--equal-split: repeat(var(--square-size), minmax(0, 1fr));
		grid-template: var(--equal-split) / var(--equal-split);
	}

	.center() {
		display: grid;
		place-items: center;
	}

	.grid {
		--square-size: v-bind(grid.square_size);
		--border-color: v-bind(props.borderColor);
		container: grid-container / inline-size;
		aspect-ratio: 1;
		inline-size: 100%;
		.square-grid();

		.square {
			.square-grid();
			--color: var(--border-color, #ccc);
			border: var(--color) solid medium;
			container: square / inline-size;
		}

		.cell {
			font-size: calc(100cqmin / var(--square-size) * 0.56);
			border-color: var(--border-color);
			--selected-color: var(--color-accent);
			--selected-color-text: var(--color-secondary);
		}
	}
</style>
