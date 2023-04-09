import { PositionalComponentBase } from '../core';
import { Content } from './Content';
import { BehaviorSubject, distinctUntilChanged, map, skip } from 'rxjs';
import { EventCallback } from '../types';

export interface SelectChangeEvent {
	target: Cell;
	isSelected: boolean;
}

export class Cell extends PositionalComponentBase<HTMLButtonElement> {
	readonly #contentElement: Content;

	readonly #selectedSubject: BehaviorSubject<boolean>;

	constructor() {
		super('button');
		this.classList.add('cell');
		this.setDataSet({ cell: true });

		this.#selectedSubject = new BehaviorSubject(false);

		this.#contentElement = new Content();
		this.#contentElement.classList.add('content');
		this.#contentElement.mount(this.element);

		this.registerEvent('focus', () => {
			console.log('focues');
			this.selected = true;
		});

		this.registerEvent('click', () => {
			console.log('click');
			this.selected = !this.selected;
		});
	}

	get value() {
		const val = this.#contentElement.value;
		if (!+val) return 0;
		return +val;
	}

	set value(value: number) {
		if (value > 0) {
			this.#contentElement.value = value.toString();
		} else {
			this.#contentElement.value = '';
		}

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

	dispose() {
		this.#selectedSubject.complete();
		super.dispose();
	}

	onSelectChange(cb: EventCallback<SelectChangeEvent>) {
		const subscription = this.#selectedSubject
			.pipe(
				skip(1),
				distinctUntilChanged(),
				map((isSelected) => ({
					isSelected,
					target: this,
				})),
			)
			.subscribe(cb);
		return () => subscription.unsubscribe();
	}
}
