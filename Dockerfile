FROM rust:1.62 AS chef
RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:buster-slim AS runtime
RUN apt update -y && apt install -y ca-certificates
WORKDIR app
COPY --from=builder /app/target/release/kafka-tailer /usr/local/bin
CMD ["/usr/local/bin/kafka-tailer"]
