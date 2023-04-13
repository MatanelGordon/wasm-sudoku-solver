export interface AnalyzedKeydownEvent {
	isUp: boolean;
	isDown: boolean;
	isLeft: boolean;
	isRight: boolean;

	isDigit: boolean;
	digit: number;
	isBackspace: boolean;
	isEscape: boolean;
	isTab: boolean;
	isDelete: boolean;
}

export const analyzeKeydownEvent = (evt: KeyboardEvent): AnalyzedKeydownEvent => {
	const key = evt.key;
	const key_code = key.charCodeAt(0);

	const isUp = key === 'ArrowUp';
	const isDown = key === 'ArrowDown';
	const isLeft = key === 'ArrowLeft';
	const isRight = key === 'ArrowRight';
	const isEscape = key === 'Escape';
	const isBackspace = key === 'Backspace';
	const isDelete = key === 'Delete';
	const isTab = key === 'Tab';
	const isDigit = key_code >= 48 && key_code <= 57;
	const digit = isDigit ? key_code - 48 : -1;

	return {
		isUp,
		isDown,
		isLeft,
		isRight,

		isTab,

		isBackspace,
		isEscape,
		isDelete,

		isDigit,
		digit,
	};
};
