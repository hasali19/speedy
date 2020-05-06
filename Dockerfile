FROM rust:latest as builder
WORKDIR /usr/src/speedy
COPY . .
RUN cargo install --path .

FROM debian:stable-slim
COPY --from=builder /usr/local/cargo/bin/speedy /usr/local/bin/speedy
CMD ["speedy"]
