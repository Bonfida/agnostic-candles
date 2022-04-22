FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR server

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /server/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin candles-server

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR server
COPY --from=builder /server/target/release/candles-server /usr/local/bin
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/candles-server"]