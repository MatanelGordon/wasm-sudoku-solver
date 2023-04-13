import type { Mat } from '@/types';
import { expect, describe, it } from 'vitest';
import { map, squareTransform } from '@/utils/matrix';

describe('matrix util', () => {
	it('should transform matrix to square', () => {
		const input: Mat<number> = [
			[1, 2, 3, 4],
			[1, 2, 3, 4],
			[1, 2, 3, 4],
			[1, 2, 3, 4]
		];

		const expected: Mat<number> = [
			[1, 2, 1, 2],
			[3, 4, 3, 4],
			[1, 2, 1, 2],
			[3, 4, 3, 4]
		];

		const result = squareTransform(input).map(group => group.map(({ value }) => value));

		expect(result).toEqual(expected);
	});

	it('should matMap and keep order', () => {
		const input: Mat<number> = [
			[1, 2],
			[3, 4]
		];
		const expected = [
			[2, 4],
			[6, 8]
		];

		const multiply = (value: number) => value * 2;

		expect(map(input, multiply)).toEqual(expected);
	});
});
