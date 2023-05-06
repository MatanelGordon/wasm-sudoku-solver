FROM 15012002/wasm-pack as builder
WORKDIR /usr/app
COPY . .
RUN make install
RUN make build

FROM nginx
WORKDIR /usr/share/nginx/html
COPY --from=builder /usr/app/dist .


