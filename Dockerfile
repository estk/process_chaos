FROM rust:1.81.0 as builder

WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} \
    cargo build --release

FROM rust:1.81.0 as runtime
WORKDIR /app
COPY --from=builder /app/target/release/chaos /usr/local/bin
ENTRYPOINT ["/usr/local/bin/chaos"]
