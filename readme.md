[![Rust](https://github.com/leomonteiro92/mini_ledger_rust/actions/workflows/rust.yml/badge.svg)](https://github.com/leomonteiro92/mini_ledger_rust/actions/workflows/rust.yml)

<p align="center">
  <img src="https://rustacean.net/assets/rustacean-flat-happy.svg" alt="Ferris the crab" width="180">
</p>

# Rust Mini Ledger
This project is a Rust-based API for managing financial operations, including account creation, deposits, withdrawals, and transfers. It supports querying account details and balances while ensuring idempotent and accurate operations. The API uses Apache Cassandra as the storage backend for its high scalability and fault tolerance.

## Features
 - **Account Management**: Create and retrieve account information.
 - **Deposits and Withdrawals**: Handle funds securely with idempotency support.
 - **Transfers**: Transfer funds between accounts.
Currency Support: Operates using a specified currency (e.g., BRL).
 - **Idempotency**: Prevent duplicate operations using idempotency keys.
 - **Cassandra Storage**: Highly scalable and distributed database for storing account and transaction data.

## Technology Stack
 - **Language**: Rust
 - **Database**: Apache Cassandra or AWS DynamoDB
 - **Frameworks**: Actix Web, Tokio, Serde

## Docs
[Open API](https://petstore.swagger.io/?url=https://raw.githubusercontent.com/leomonteiro92/mini_ledger_rust/refs/heads/main/openapi.yml)

## Troubleshooting build
### Mac OSx
```bash
brew install \
    bash \
    curl \
    cmake \
    openssl \
    libuv \
    zlib \
    git
```
Then build the app with the following command
```bash
export RUSTFLAGS="-L /opt/homebrew/opt/openssl@3/lib -L /opt/homebrew/opt/cassandra-cpp-driver/lib -L /opt/homebrew/opt/libuv/lib" && cargo build
```
