import type { WatchOptions, WatchSource, WatchStopHandle } from 'vue';
import { ref, watch } from 'vue';

type WatchDisableHandle = (force?: boolean) => void;

export const controlledWatchEffect = <T, Immediate extends Readonly<boolean> = false>(
	source: WatchSource<T> | object,
	cb: (val: T | object) => void,
	options?: WatchOptions<Immediate>
): [WatchDisableHandle, WatchStopHandle] => {
	const isDisabled = ref(false);
	const disableWatch: WatchDisableHandle = (force?: boolean) => {
		const prev = isDisabled.value;
		isDisabled.value = force ?? !prev;
	};
	const cb_wrapper: typeof cb = new_val => {
		if (isDisabled.value) {
			return;
		}
		cb(new_val);
	};
	const unwatch = watch(source, cb_wrapper, options);
	return [disableWatch, unwatch];
};
