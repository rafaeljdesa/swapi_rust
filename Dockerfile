FROM rustlang/rust:nightly as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 libmariadb3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/swapi_rust /app/swapi_rust
COPY config/ ./config/
COPY sql/ ./sql/
EXPOSE 8080
CMD ["/app/swapi_rust"]
