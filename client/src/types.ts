export type Mat<T> = Array<Array<T>>;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type ComponentEvent<Evt extends (...args: any[]) => unknown> = Exclude<
	Parameters<Evt>[0],
	`update:${string}`
>;
