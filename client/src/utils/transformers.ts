export interface Transformer<TFrom, TTo = string> {
	from(val: TTo | null): TFrom | null;

	to(val: TFrom): TTo;
}

export const NUMERICAL_TRANSFORMER: Transformer<number> = {
	from(val: string | null): number | null {
		if (val === null) return null;
		return +val;
	},
	to(val: number): string {
		return val.toString();
	}
};
