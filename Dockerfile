# Build stage
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/simple-server ./server
EXPOSE 8080
CMD ["./server"]
