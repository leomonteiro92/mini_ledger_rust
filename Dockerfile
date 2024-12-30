# Start from Alpine base image
FROM alpine:3.18

# Set environment variables for Rust and Cargo
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# Install required dependencies
RUN apk add --no-cache \
    bash \
    curl \
    build-base \
    cmake \
    openssl-dev \
    libuv-dev \
    zlib-dev \
    git \
    g++

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y && \
    rustup default stable

# Clone and build the Cassandra CPP driver
RUN git clone --depth 1 https://github.com/datastax/cpp-driver.git /tmp/cpp-driver && \
    mkdir /tmp/cpp-driver/build && \
    cd /tmp/cpp-driver/build && \
    cmake .. -DCMAKE_INSTALL_PREFIX=/usr/local && \
    make -j$(nproc) && \
    make install && \
    ldconfig

# Default command
CMD ["cargo", "build", "--verbose"]
