export type Mat<T> = Array<Array<T>>;
export type ComponentEvent<Evt extends (...args: any[]) => unknown> = Exclude<
	Parameters<Evt>[0],
	`update:${string}`
>;
