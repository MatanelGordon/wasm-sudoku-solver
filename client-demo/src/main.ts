import './style/index.less';
import { solve } from 'client-lib';
import { Grid } from './components/Grid';

const root = document.querySelector<HTMLDivElement>('#app')!;

const board = [
	[0, 0, 0, 8, 0, 0, 6, 1, 0],
	[2, 1, 0, 0, 0, 0, 0, 5, 0],
	[0, 0, 4, 0, 0, 0, 0, 0, 7],
	[0, 0, 0, 9, 0, 0, 0, 8, 0],
	[0, 0, 0, 1, 6, 0, 0, 0, 0],
	[1, 0, 7, 0, 0, 8, 0, 3, 0],
	[0, 0, 0, 3, 0, 0, 0, 0, 0],
	[0, 0, 9, 0, 0, 0, 0, 0, 4],
	[0, 7, 0, 5, 0, 9, 2, 0, 1],
];

const raw_data = Uint32Array.from(board.flat());
const grid = new Grid();

grid.mount(root);

const res = solve(raw_data);

grid.load(raw_data);

console.log(res);
