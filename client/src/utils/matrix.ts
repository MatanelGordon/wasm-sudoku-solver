import type { Mat } from '@/types';
import { roundSqrt } from '@/utils/math';

export interface SquareValue<T> {
	row: number;
	col: number;
	value: T;
}

export const squareTransform = <T>(data: Mat<T>): Mat<SquareValue<T>> => {
	const size = data.length;
	const square_size = roundSqrt(size);
	const squares: Mat<SquareValue<T>> = new Array(size).fill(null).map(() => new Array(size));

	for (let square_row = 0; square_row < square_size; square_row++) {
		for (let square_col = 0; square_col < square_size; square_col++) {
			const square_index = square_row * square_size + square_col;
			for (let inner_row = 0; inner_row < square_size; inner_row++) {
				for (let inner_col = 0; inner_col < square_size; inner_col++) {
					const row = square_row * square_size + inner_row;
					const col = square_col * square_size + inner_col;
					const cell_index = inner_row * square_size + inner_col;
					const value = data[row][col];
					squares[square_index][cell_index] = {
						col,
						row,
						value
					};
				}
			}
		}
	}

	return squares;
};

export const matMap = <T, U>(matrix: Mat<T>, cb: (value: T) => U): Mat<U> => {
	return matrix.map(group => group.map(cb));
};
