import { ComponentBase } from '../core';
import { Cell } from './Cell';
import { Square } from './Square';
import { analyzeKeydownEvent, registerEvent, removeChildren } from '../utils';
import { analyzeClickEvent } from '../logic';
import { DisposeFn } from '../types';

export class Grid extends ComponentBase<HTMLDivElement> {
	#data: Array<Uint32Array> = [];
	#cells: Array<Array<Cell>> = [];

	#selected_cells: Set<Cell> = new Set();

	#disposeEvents?: DisposeFn;

	constructor() {
		super('div');
		this.reset();
		this.classList.add('grid-layout');
	}

	get size() {
		return this.#data.length;
	}

	get square_size() {
		return Math.sqrt(this.size) | 0;
	}

	load(data: Uint32Array | Array<Array<number> | Uint32Array>) {
		if (data.length === 0) {
			throw new Error('Could not load empty data');
		}

		if (data instanceof Uint32Array) {
			const size = Math.sqrt(data.length) | 0;
			this.#data = new Array(size)
				.fill(null)
				.map((_, i) => data.slice(i * size, (i + 1) * size));
		} else {
			this.#data = data.map(Uint32Array.from);
		}

		this.render();
	}

	loadEmpty(size: number) {
		if (size < 4) {
			throw new Error('Size too small');
		}
		this.#data = new Array(size).fill(null).map(() => new Uint32Array(size));
		this.render();
	}

	render() {
		const prevSize = this.#cells.length;
		if (prevSize !== this.size) {
			this.buildGrid();
		}
	}

	private createCell(row: number, col: number) {
		const size = this.size;
		const cell = new Cell();

		cell.col = col;
		cell.row = row;
		cell.tabIndex = row * size + col + 1;

		cell.onselect = ({ target, selected }) => {
			if (selected) {
				this.#selected_cells.add(target);
			} else {
				this.#selected_cells.delete(target);
			}
		};

		return cell;
	}

	private buildGrid() {
		this.reset();
		const size = this.size;
		const s_size = this.square_size;
		this.#cells = new Array(size).fill(null).map(() => []);

		this.setCSSVariable('square-size', s_size);

		for (let s_row = 0; s_row < s_size; s_row++) {
			for (let s_col = 0; s_col < s_size; s_col++) {
				const square = new Square();
				for (let inner_row = 0; inner_row < s_size; inner_row++) {
					for (let inner_col = 0; inner_col < s_size; inner_col++) {
						const row = s_row * s_size + inner_row;
						const col = s_col * s_size + inner_col;
						const cell = this.createCell(row, col);
						cell.value = this.#data[row][col] ?? 0;
						this.#cells[row][col] = cell;
						cell.mount(square);
					}
				}
				square.mount(this);
			}
		}
		this.initEvents();
	}

	private selectCell(row: number, col: number, value?: boolean) {
		const cell = this.#cells[row][col];

		if (!cell || (value !== undefined && cell.selected === value)) return;

		cell.selected = value ?? !cell.selected;
		console.log(`cell(${row}, ${col}) -> ${cell.selected}`);
	}

	private clearAllSelected(exceptions: Array<Cell> = []) {
		for (const cell of this.#selected_cells) {
			if (exceptions.includes(cell)) continue;
			cell.selected = false;
		}
	}

	private setValue(value: number, initial = true) {
		for (const cell of this.#selected_cells) {
			if (initial) {
				cell.value = value;
			} else {
				const prev = cell.value;
				const next = prev * 10 + value;

				if (next > this.size) {
					cell.value = value;
				} else {
					cell.value = next;
				}
			}
		}
	}

	private initEvents() {
		let clicked = false;
		const disposeKeydown = registerEvent(window, 'keydown', evt => {
			//todo: arrow movement using selectCell with singular

			const {
				isTab,
				isEscape,
				isDigit,
				digit,
				isBackspace,
				isDelete,
				isDown,
				isUp,
				isLeft,
				isRight,
			} = analyzeKeydownEvent(evt);

			console.log(evt);
			const lastSelected = Array.from(this.#selected_cells).at(-1) ?? this.#cells[0][0];
			const currRow = lastSelected.row;
			const currCol = lastSelected.col;

			// At Tab unselect all and then the individual "focus" will take over
			if (isTab) {
				this.clearAllSelected();
			}
			// At Escape key unselect all
			else if (isEscape) {
				this.clearAllSelected();
			}
			// At any digit key set value appropriately
			else if (isDigit) {
				this.setValue(digit, clicked);
			}
			// At Backspace key undo value
			else if (isBackspace) {
				for (const cell of this.#selected_cells) {
					cell.value = (cell.value / 10) | 0;
				}
			}
			// At Delete key delete value
			else if (isDelete) {
				for (const cell of this.#selected_cells) {
					cell.value = 0;
				}
			}
			// At Up key move up
			else if (isUp) {
				this.clearAllSelected();
				const row = currRow - 1;
				const nextRow = row < 0 ? this.size - 1 : row;
				this.selectCell(nextRow, currCol, true);
			}
			// At Down key move down
			else if (isDown) {
				this.clearAllSelected();
				this.selectCell((currRow + 1) % this.size, currCol, true);
			}
			// At Left key move left
			else if (isLeft) {
				this.clearAllSelected();
				const col = currCol - 1;
				const nextCol = col < 0 ? this.size - 1 : col;
				this.selectCell(currRow, nextCol, true);
			}
			// At right key move right
			else if (isRight) {
				this.clearAllSelected();
				this.selectCell(currRow, (currCol + 1) % this.size, true);
			}

			clicked = false;
		});

		const disposeClick = registerEvent(window, 'click', (evt: MouseEvent) => {
			console.log('click');
			clicked = true;

			const { row, col, isCell, isMultiplier } = analyzeClickEvent(evt);

			if (!isCell) {
				this.clearAllSelected();
				return;
			}

			const cell = this.#cells[row][col];

			if (!isMultiplier) {
				this.clearAllSelected([cell]);
			}

			this.selectCell(row, col, true);
		});

		this.#disposeEvents = () => {
			disposeKeydown();
			disposeClick();
		};
	}

	reset() {
		removeChildren(this.element);
		this.#cells = [];
		this.#disposeEvents?.();
		this.#selected_cells.clear();
		this.#disposeEvents = undefined;
	}
}
