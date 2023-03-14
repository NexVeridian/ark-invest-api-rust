FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl && apt update && apt install -y musl-tools musl-dev

WORKDIR /ark-invest-api-rust

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo,from=rust,source=/usr/local/cargo \
	--mount=type=cache,target=./target \
	cargo build --target x86_64-unknown-linux-musl --release && \
	cp ./target/x86_64-unknown-linux-musl/release/ark-invest-api-rust .

FROM alpine:latest

WORKDIR /ark-invest-api-rust

COPY --from=builder ark-invest-api-rust/ark-invest-api-rust .

ENV PORT=3000
EXPOSE 3000

CMD ["./ark-invest-api-rust"]
