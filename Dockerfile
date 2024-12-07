FROM 15012002/wasm-pack:node-22-rust-1.83 AS builder
WORKDIR /usr/app
COPY . .
RUN make install
RUN make build

FROM nginx:1.26.2-alpine
WORKDIR /usr/share/nginx/html
COPY --from=builder /usr/app/dist .


