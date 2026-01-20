FROM rust:1 AS builder
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl
COPY . .
ARG TARGET
RUN if [ -z "$TARGET" ]; then \
        ARCH=$(uname -m); \
        if [ "$ARCH" = "x86_64" ]; then \
            RUST_TARGET=x86_64-unknown-linux-musl; \
        elif [ "$ARCH" = "aarch64" ]; then \
            RUST_TARGET=aarch64-unknown-linux-musl; \
        else \
            echo "Unsupported architecture: $ARCH"; exit 1; \
        fi; \
    else \
        if [ "$TARGET" = "linux/amd64" ]; then \
            RUST_TARGET=x86_64-unknown-linux-musl; \
        elif [ "$TARGET" = "linux/arm64" ]; then \
            RUST_TARGET=aarch64-unknown-linux-musl; \
        else \
            echo "Unsupported TARGET: $TARGET"; exit 1; \
        fi; \
    fi && \
    cargo build --release --target $RUST_TARGET



FROM scratch
WORKDIR /app
COPY --from=builder /usr/src/app/target/*/release/guestbook .
COPY static/ static/
ENTRYPOINT ["./guestbook"]