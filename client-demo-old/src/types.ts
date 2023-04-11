export type Stringable = number | string | boolean;
export type EventMap = GlobalEventHandlersEventMap;

export type EventCallback<T> = (payload: T) => unknown;

export type HtmlEventCallback<T extends keyof EventMap> = EventCallback<EventMap[T]>;

export type DisposeFn = () => void;
