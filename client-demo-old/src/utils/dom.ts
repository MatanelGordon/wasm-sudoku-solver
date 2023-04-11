import { DisposeFn, HtmlEventCallback, EventMap } from '../types';

export const removeChildren = (element: HTMLElement) => {
	while (element.firstChild) {
		element.firstChild.remove();
	}
};

export const registerEvent = <T extends keyof EventMap>(
	target: GlobalEventHandlers,
	event: T,
	cb: HtmlEventCallback<T>,
): DisposeFn => {
	target.addEventListener(event, cb);
	return () => {
		target.removeEventListener(event, cb);
	};
};
