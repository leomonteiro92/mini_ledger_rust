# Start from Alpine base image
FROM rust:1.89-alpine

# Install required dependencies
RUN apk add --no-cache \
    bash \
    curl \
    musl \
    musl-dev \
    cassandra-cpp-driver \
    cassandra-cpp-driver-dev

COPY . .

# Default command
RUN cargo build --release

CMD ["cargo", "run", "--release"]
