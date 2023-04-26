import type { Mat } from '@/types';

export type MatValue = number | { value: number };
export const resolveMatValue = (val: MatValue): number =>
	typeof val === 'object' ? val.value : val;

export const flatMatrix = (matrix: Mat<MatValue>): Uint32Array =>
	Uint32Array.from(matrix.flat().map(resolveMatValue));
