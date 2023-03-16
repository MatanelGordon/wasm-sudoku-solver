install:
	cargo update && npm install

test-core:
	cargo test -p core --lib

build-lib:
	wasm-pack build ./client-lib