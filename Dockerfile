FROM rust:1 AS builder
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl
COPY . .
ARG TARGETPLATFORM
RUN if [ -z "$TARGETPLATFORM" ]; then \
        ARCH=$(uname -m); \
        if [ "$ARCH" = "x86_64" ]; then \
            RUST_TARGET=x86_64-unknown-linux-musl; \
        elif [ "$ARCH" = "aarch64" ]; then \
            RUST_TARGET=aarch64-unknown-linux-musl; \
        else \
            echo "Unsupported architecture: $ARCH"; exit 1; \
        fi; \
    else \
        if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
            RUST_TARGET=x86_64-unknown-linux-musl; \
        elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
            RUST_TARGET=aarch64-unknown-linux-musl; \
        else \
            echo "Unsupported TARGETPLATFORM: $TARGETPLATFORM"; exit 1; \
        fi; \
    fi && \
    echo "Building Rust target: $RUST_TARGET" && \
    cargo build --release --target $RUST_TARGET


FROM scratch
WORKDIR /app
COPY --from=builder /usr/src/app/target/*/release/guestbook .
COPY static/ static/
ENTRYPOINT ["./guestbook"]