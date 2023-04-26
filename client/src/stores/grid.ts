import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import type { Mat } from '@/types';
import { forEach, map } from '@/utils';
import { roundSqrt } from '@/utils/math';

export interface StoreData {
	readonly row: number;
	readonly col: number;
	value: number;
	selected: boolean;
}

export const useGridStore = defineStore('grid', () => {
	const data = ref<Mat<StoreData>>([]);

	const size = computed(() => data.value.length);
	const square_size = computed(() => roundSqrt(size.value));
	const selected_cells = computed(() => data.value.flat().filter(x => x.selected));
	const last_selected = computed(() => selected_cells.value.at(-1));

	function get(row: number, col: number): StoreData | undefined {
		return data.value[row][col];
	}

	function setValue(row: number, col: number, value: number) {
		data.value[row][col].value = value;
	}

	function setSelected(row: number, col: number, selected = true, clear_rest = true) {
		data.value[row][col].selected = selected;

		if (clear_rest) {
			clearAllSelected([[row, col]]);
		}
	}

	function setSelectedValue(value: number | ((prev: number) => number)) {
		selected_cells.value.forEach(({ row, col }) => {
			const prev = data.value[row][col].value;

			data.value[row][col].value = typeof value === 'function' ? value(prev) : value;
		});
	}

	function load(matrix: Mat<number>) {
		data.value = map(
			matrix,
			(value, row, col): StoreData => ({
				row,
				col,
				value,
				selected: false
			})
		);
	}

	function clearAllSelected(exceptions: (StoreData | [number, number])[] = []) {
		forEach(data.value, item => {
			const isException = exceptions.some((value: StoreData | [number, number]) => {
				let row: number, col: number;

				if (value instanceof Array) {
					row = value[0];
					col = value[1];
				} else {
					row = value.row;
					col = value.col;
				}

				return item.row === row && item.col === col;
			});

			if (isException) return;
			item.selected = false;
		});
	}

	return {
		data,
		size,
		square_size,
		selected_cells,
		last_selected,
		get,
		setValue,
		setSelected,
		load,
		clearAllSelected,
		setSelectedValue
	};
});

export type GridStore = ReturnType<typeof useGridStore>;
