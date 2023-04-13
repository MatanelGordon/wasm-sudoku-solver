export type Mat<T> = Array<Array<T>>;
export type ComponentEvents<Evt extends (...args: any[]) => unknown> = Parameters<Evt>[0];
