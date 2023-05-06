install:
	cargo update && npm install

test-core:
	cargo test -p core --lib

build-core: test-core
	cargo build

build-wasm: build-core
	wasm-pack build ./wasm-lib

run-dev: build-wasm
	npm run dev -w client

build: build-wasm
	npm run build -w client

clean:
	rm -rf **/target **/dist **/build