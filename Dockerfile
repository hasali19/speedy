FROM rust:latest as rust-build
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM node:latest as node-build
WORKDIR /app
COPY client/ .
RUN npm install && npm run build

FROM debian:stable-slim
WORKDIR /app
COPY --from=rust-build /usr/local/cargo/bin/speedy ./speedy
COPY --from=node-build /app/build ./client/build
CMD ["./speedy"]
