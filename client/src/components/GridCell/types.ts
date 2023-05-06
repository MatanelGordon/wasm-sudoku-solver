export interface CellEventPayload<Evt extends Event> {
	row: number;
	col: number;
	selected: boolean;
	event: Evt;
}
