import type { Transformer } from '@/utils';
import { ref, watchEffect } from 'vue';

export const useStorageState = <T extends number | boolean | string>(
	key: string,
	initialValue: T,
	transformer: Transformer<T>
) => {
	const current = ref(transformer.from(window.localStorage.getItem(key)) ?? initialValue);

	watchEffect(() => {
		if (!current.value) return;
		window.localStorage.setItem(key, transformer.to(current.value as T));
	});

	return current;
};
