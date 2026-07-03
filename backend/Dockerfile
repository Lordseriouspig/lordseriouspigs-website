FROM rust:1.96-bookworm as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/backend /app/server

EXPOSE 4000

RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

CMD ["./server"]