import { DisposeFn, HtmlEventCallback, EventMap, Stringable } from '../types';
import { registerEvent } from '../utils';

export abstract class ComponentBase<P extends HTMLElement = HTMLElement> {
	protected readonly element: P;

	protected constructor(tagName: string) {
		this.element = document.createElement(tagName) as P;
	}

	get classList() {
		return this.element.classList;
	}

	setAttribute(key: string, value: Stringable) {
		this.element.setAttribute(key, value.toString());
	}

	setDataSet(dataset: Record<string, Stringable>) {
		Object.entries(dataset).forEach(([key, value]) => {
			this.setAttribute(`data-${key}`, value.toString());
		});
	}

	getData(key: string) {
		return this.element.dataset[`data-${key}`];
	}

	registerEvent<P extends keyof EventMap>(event: P, cb: HtmlEventCallback<P>): DisposeFn {
		return registerEvent<P>(this.element, event, cb);
	}

	mount(container: HTMLElement | ComponentBase) {
		if (container instanceof ComponentBase) {
			container.element.appendChild(this.element);
			return;
		}
		container.appendChild(this.element);
	}

	dispose() {
		this.element.remove();
	}
}
