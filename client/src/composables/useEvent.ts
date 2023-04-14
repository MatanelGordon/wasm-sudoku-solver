import { onMounted, onUnmounted } from 'vue';

export function useEvent<E extends keyof GlobalEventHandlersEventMap>(
	target: GlobalEventHandlers,
	eventName: E,
	cb: (payload: GlobalEventHandlersEventMap[E]) => unknown
) {
	onMounted(() => {
		target.addEventListener(eventName, cb);
	});

	onUnmounted(() => {
		target.removeEventListener(eventName, cb);
	});
}
