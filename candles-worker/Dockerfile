FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR worker

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /worker/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY src Cargo.toml ./
RUN cargo build --release --bin candles-worker

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR worker
COPY --from=builder /worker/target/release/candles-worker /usr/local/bin
ENTRYPOINT ["/usr/local/bin/candles-worker"]