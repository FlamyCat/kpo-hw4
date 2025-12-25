FROM rust:1.92 as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.92 as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.92 as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
RUN cargo build --release --workspace

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ARG APP_NAME

COPY --from=builder /app/target/release/${APP_NAME} /app/service

ENV IS_DOCKER=true
ENV RUST_LOG=info

CMD ["./service"]
