import { ComponentBase } from '../core';

export class Content extends ComponentBase<HTMLSpanElement> {
	#value: string;
	constructor() {
		super('span');

		this.#value = '';

		this.element.style.pointerEvents = 'none';
	}

	get value() {
		return this.#value;
	}

	set value(value) {
		this.#value = value;
		this.element.textContent = value;
	}
}
