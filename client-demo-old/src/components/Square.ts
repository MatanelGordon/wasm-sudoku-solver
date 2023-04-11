import { ComponentBase } from '../core';

export class Square extends ComponentBase<HTMLDivElement> {
	constructor() {
		super('div');
		this.setAttribute('aria-hidden', true);
		this.element.tabIndex = -1;
		this.classList.add('square');
	}
}
