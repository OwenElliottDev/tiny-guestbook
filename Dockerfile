FROM rust:1 AS builder
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cross
WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
WORKDIR /app
COPY static/ static/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/guestbook .
ENTRYPOINT ["./guestbook"]
