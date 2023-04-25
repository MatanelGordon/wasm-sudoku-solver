# WebAssembly Sudoku Solver
![GitHub](https://img.shields.io/github/license/MatanelGordon/wasm-sudoku-solver?style=for-the-badge)

Sudoku Solver using WASM and Vue3 Composition API.

<img width="684" alt="Screenshot 2023-04-25 at 19 18 46" src="https://user-images.githubusercontent.com/72258277/234340119-0a864ef5-7af9-44e5-b94b-c7e5f6822a89.png">

## The Project

This is a rust+node monorepo with several packages:

- `core` - handles the business logic layer (the solver itself + UT)
- `wasm-lib` - a wasm wrapper for `core`
- `client` - the UI itself powered by Vue3 that uses the `wasm-lib`

## Usage
 
to install the required dependencies, use the makefile:

```bash
make install
```

### Client Dev Server 
to run the client dev server run the following:

```bash
make dev-server
```

> Note: Running this will initially build both the `core` and `wasm-lib`, so no need to run further commands afterwards

### `core` Library

to run the tests for the core library run the following:

```bash
make test-core
```

to build the core library after changes do the following:

```bash
make build-core
```

### Handling `wasm-lib`

to build the wasm-lib after changes run the following: 

```bash
make build-lib
```

> NOTE: `build-lib` also builds the `core` and will 

## Contributing

Submit an issue, and after discussing it, fill free to open a new branch and submit a PR. 

`feature/*` for branches that add something new

`bugfix/*` for branches that fix something broken

`refactor/*` that's more for me :)
