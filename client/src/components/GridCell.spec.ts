import { it, describe, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import GridCell from '@/components/GridCell.vue';

describe('Grid Cell tests', () => {
	it('should have active class when selected=true', () => {
		const comp = mount(GridCell, { props: { row: 0, col: 0, selected: true } });
		expect(comp.classes()).toContain('active');
	});

	it('should have proper tabIndex', () => {
		const row = 1;
		const col = 2;
		const size = 3;
		const expectedTabIndex = row * size + col + 1;
		const comp = mount(GridCell, { props: { row, col, size } });
		expect(+comp.attributes()['tabindex']).toBe(expectedTabIndex);
	});

	it.each([
		['focus', 'focus'],
		['click', 'mousedown']
	])('should trigger %s', (toEmit, emitted) => {
		const comp = mount(GridCell, { props: { row: 0, col: 0 } });
		comp.trigger(emitted);
		const emittedEvents = comp.emitted();
		expect(emittedEvents).toHaveProperty(toEmit);
	});
});
