install:
	cargo update && npm install

test-core:
	cargo test -p core --lib

build-core: test-core
	cargo build

build-lib: build-core
	wasm-pack build ./wasm-lib

run-dev: build-lib
	npm run dev -w client

clean:
	rm -rf **/target **/dist **/build