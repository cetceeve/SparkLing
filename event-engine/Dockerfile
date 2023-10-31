FROM rust:bookworm as builder
WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/event-engine /usr/local/bin/event-engine
CMD ["event-engine"]