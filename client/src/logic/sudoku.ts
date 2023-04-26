import type { Mat } from '@/types';
import { randomRange, roundSqrt } from '@/utils/math';
import { solve as wasmSolve, is_valid } from 'wasm-lib';
import { createSquareMatrix } from '@/utils/matrix';
import { flatMatrix } from '@/logic/utils';
import type { MatValue } from '@/logic/utils';

export const solve = (matrix: Mat<MatValue>): Mat<number> => {
	const flat_grid = flatMatrix(matrix);
	const flat_solved = wasmSolve(flat_grid);
	const size = roundSqrt(flat_solved.length);
	return new Array(size)
		.fill(null)
		.map((_, i) => Array.from(flat_solved.slice(i * size, (i + 1) * size)));
};

export const isValid = (matrix: Mat<MatValue>) => {
	const flat_grid = flatMatrix(matrix);
	return is_valid(flat_grid);
};

export const generateSudokuBoard = (size: number): Mat<number> => {
	const options = Array.from(new Array(size).keys());
	const board = createSquareMatrix(size, 0);

	for (const num of options) {
		let row: number, col: number;

		do {
			row = randomRange(0, size);
			col = randomRange(0, size);
		} while (board[row][col] !== 0);

		board[row][col] = num;
	}

	return board;
};
