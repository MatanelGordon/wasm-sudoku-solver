import { Cell } from '../components/Cell';

export interface AnalyzedClickEvent {
	target: HTMLElement | null;
	isCell: boolean;
	row?: number;
	col?: number;
	value?: number;
}

export const analyzeClickEvent = (
	evt: Event,
): (Required<AnalyzedClickEvent> & { isCell: true }) | (AnalyzedClickEvent & { isCell: false }) => {
	const target = evt.target as HTMLElement | null;
	const isCell = !!target && Cell.isCell(target);

	return {
		isCell,
		target,
		col: +(target?.dataset?.['col'] ?? -1),
		row: +(target?.dataset?.['row'] ?? -1),
		value: +(target?.dataset?.['row'] ?? -1),
	};
};
