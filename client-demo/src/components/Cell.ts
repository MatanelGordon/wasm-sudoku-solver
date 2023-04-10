import { PositionalComponentBase } from '../core';
import { Content } from './Content';
import { BehaviorSubject, distinctUntilChanged, map, skip } from 'rxjs';
import { EventCallback } from '../types';

export interface CellEvent {
	target: Cell;
	selected: boolean;
	row: number;
	col: number;
	value: number;
}

export class Cell extends PositionalComponentBase<HTMLButtonElement> {
	readonly #contentElement: Content;

	readonly #selectedSubject: BehaviorSubject<boolean>;
	readonly #valueSubject: BehaviorSubject<number>;

	constructor() {
		super('button');
		this.classList.add('cell');
		this.setDataSet({ cell: true });

		this.#selectedSubject = new BehaviorSubject(false);
		this.#valueSubject = new BehaviorSubject(0);

		this.#contentElement = new Content();
		this.#contentElement.classList.add('content');
		this.#contentElement.mount(this.element);

		this.registerEvent('focus', () => {
			console.log('focus');
			this.selected = true;
		});

		this.onvalue = ({ value }) => {
			if (value > 0) {
				this.#contentElement.value = value.toString();
			} else {
				this.#contentElement.value = '';
			}
		};
	}

	get value() {
		return this.#valueSubject.value;
	}

	set value(value: number) {
		this.#valueSubject.next(value);
		this.setDataSet({ value });
	}

	get tabIndex() {
		return this.element.tabIndex;
	}

	set tabIndex(value: number) {
		this.element.tabIndex = value;
	}

	get selected() {
		return this.#selectedSubject.value;
	}

	set selected(value) {
		this.#selectedSubject.next(value);
		this.classList.toggle('active', value);
	}

	static isCell(el: HTMLElement) {
		return el.dataset['cell'];
	}

	protected get payload(): CellEvent {
		return {
			target: this,
			col: this.col,
			row: this.row,
			value: this.value,
			selected: this.selected,
		};
	}

	set onselect(cb: EventCallback<CellEvent>) {
		this.#selectedSubject
			.pipe(
				skip(1),
				distinctUntilChanged(),
				map(() => this.payload),
			)
			.subscribe(cb);
	}

	set onvalue(cb: EventCallback<CellEvent>) {
		this.#valueSubject
			.pipe(
				distinctUntilChanged(),
				map(() => this.payload),
			)
			.subscribe(cb);
	}

	dispose() {
		this.#selectedSubject.complete();
		this.#valueSubject.complete();
		super.dispose();
	}
}
