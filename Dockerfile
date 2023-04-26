FROM rust as wasm-build
WORKDIR /usr/rust
COPY ./core ./core
COPY ./wasm-lib ./wasm
RUN make build-lib

