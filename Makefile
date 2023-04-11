install:
	cargo update && npm install

test-core:
	cargo test -p core --lib

build-core:
	cargo build

build-lib:
	wasm-pack build ./wasm-lib

run-dev:
	npm run dev -w client

clean:
	rm -rf **/target **/dist **/build