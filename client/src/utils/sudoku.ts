import type { Mat } from '@/types';
import { roundSqrt } from '@/utils/math';
import { solve as wasmSolve } from 'wasm-lib';

type MatValue = number | { value: number };
const resolveMatValue = (val: MatValue): number => (typeof val === 'object' ? val.value : val);
export const solve = (matrix: Mat<MatValue>): Mat<number> => {
	const flat_grid = Uint32Array.from(matrix.flat().map(resolveMatValue));
	const flat_solved = wasmSolve(flat_grid);
	const size = roundSqrt(flat_solved.length);
	return new Array(size)
		.fill(null)
		.map((_, i) => Array.from(flat_solved.slice(i * size, (i + 1) * size)));
};
