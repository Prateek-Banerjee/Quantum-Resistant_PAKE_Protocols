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

RUN cargo build --release --bin server

# ---------- Runtime stage ----------
FROM ubuntu:24.04

RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl3 && \
    rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --user-group server_user

WORKDIR /app

COPY --from=builder /app/test_framework_api/target/release/server /app/server

RUN chown server_user:server_user /app/server

USER server_user

EXPOSE 8080

ENTRYPOINT [ "/app/server" ]