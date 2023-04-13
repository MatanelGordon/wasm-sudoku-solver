<script setup lang="ts">
	import { Mat } from '@/types';
	import { computed } from 'vue';
	import { squareTransform } from '@/utils/matrix';
	import { roundSqrt } from '@/utils/math';
	import GridCell from '@/components/GridCell.vue';

	interface GridProps {
		data: Mat<number>;
		borderColor?: string;
	}

	const props = defineProps<GridProps>();

	const square_size = computed(() => roundSqrt(props.data.length));
	const squares = computed(() => squareTransform(props.data));
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
				class-name="cell"
				:size="props.data.length"
				:col="cell.col"
				:row="cell.row"
				:value="cell.value"
				:key="`c-${i}-${j}-${cell.value}`"
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
		--square-size: v-bind(square_size);
		container: grid-container / inline-size;
		aspect-ratio: 1;
		block-size: 80vmin;
		.square-grid();

		.square {
			.square-grid();
			border: #ccc solid medium;
			container: square / inline-size;
		}

		.cell {
			font-size: calc(100cqi / var(--square-size) * 0.56);
			--color: #ccc;
			--selected-color: var(--vt-c-accent);
		}
	}
</style>
