# Build app and run migrations
FROM rust AS builder
WORKDIR /app/
COPY . .
RUN cargo build --release
RUN cargo install diesel_cli --no-default-features --features postgres
CMD ["diesel setup && diesel migration run"]

# Run app
FROM ubuntu:22.04 AS executor
RUN apt-get update && apt-get install -y libpq-dev && apt clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/habitus_habits /

CMD ["/habitus_habits"]
EXPOSE 3030