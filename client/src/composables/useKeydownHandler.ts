import type { GridStore } from '@/stores/grid';
import { analyzeKeydownEvent } from '@/utils';
import { useEvent } from '@/composables/useEvent';

export function useKeydownHandler(grid: GridStore) {
	function keydownHandler(evt: KeyboardEvent) {
		const { isEscape, isDelete, isDigit, digit, isBackspace, isDown, isLeft, isUp, isRight } =
			analyzeKeydownEvent(evt);
		if (isEscape) {
			window.focus();
			grid.clearAllSelected();
		} else if (isDelete) {
			grid.setSelectedValue(0);
		} else if (isDigit) {
			grid.setSelectedValue(prev => {
				const nextValue = prev * 10 + digit;
				return nextValue > grid.size ? digit : nextValue;
			});
		} else if (isBackspace) {
			for (const { row, col, value } of grid.selected_cells) {
				const nextValue = (value / 10) | 0;
				grid.setValue(row, col, nextValue);
			}
		} else if (isDown) {
			const { row, col } = grid.last_selected ?? grid.data[0][0];
			let nextRow = row + 1;
			nextRow = nextRow >= grid.size ? 0 : nextRow;
			grid.setSelected(nextRow, col);
		} else if (isUp) {
			const { row, col } = grid.last_selected ?? grid.data[0][0];
			let nextRow = row - 1;
			nextRow = nextRow < 0 ? grid.size - 1 : nextRow;
			grid.setSelected(nextRow, col);
		} else if (isRight) {
			const { row, col } = grid.last_selected ?? grid.data[0][0];
			let nextCol = col + 1;
			nextCol = nextCol >= grid.size ? 0 : nextCol;
			grid.setSelected(row, nextCol);
		} else if (isLeft) {
			const { row, col } = grid.last_selected ?? grid.data[0][0];
			let nextCol = col - 1;
			nextCol = nextCol < 0 ? grid.size - 1 : nextCol;
			grid.setSelected(row, nextCol);
		}
	}

	useEvent(window, 'keydown', keydownHandler);
}
