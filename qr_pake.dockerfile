# ---------- Build stage ----------
FROM ubuntu:24.04 AS builder

RUN apt-get update && \
    apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY ./qr_pake_protocols/ ./qr_pake_protocols/
COPY ./test_framework_api/ ./test_framework_api/

# Build the app
WORKDIR /app/test_framework_api

RUN cargo build --bin server
RUN cargo build --bin server_db_storage

# ---------- Runtime stage ----------
FROM ubuntu:24.04

RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/test_framework_api/target/debug/server /app/server

EXPOSE 8080

CMD ["/app/server"]