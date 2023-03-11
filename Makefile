install:
	cargo update

test-core:
	cargo test -p core --lib

build-lib:
	wasm-pack build ./client-lib


