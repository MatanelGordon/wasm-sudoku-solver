import { Cell } from '../components/Cell';

export interface AnalyzedClickEvent {
	target: HTMLElement | null;
	isCell: boolean;
	isMultiplier: boolean;
	row?: number;
	col?: number;
	value?: number;
}

export const analyzeClickEvent = (
	evt: MouseEvent,
): (Required<AnalyzedClickEvent> & { isCell: true }) | (AnalyzedClickEvent & { isCell: false }) => {
	const target = evt.target as HTMLElement | null;
	const isCell = !!target && Cell.isCell(target);
	const isMultiplier = evt.metaKey || evt.ctrlKey;

	return {
		isCell,
		target,
		isMultiplier,
		col: +(target?.dataset?.['col'] ?? -1),
		row: +(target?.dataset?.['row'] ?? -1),
		value: +(target?.dataset?.['row'] ?? -1),
	};
};
