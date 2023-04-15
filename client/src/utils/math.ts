export const roundSqrt = (value: number) => Math.sqrt(value) | 0;

export const randomRange = (min: number, max: number) => (min + Math.random() * (max - min)) | 0;
