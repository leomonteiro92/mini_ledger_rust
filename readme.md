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
 - **Database**: Apache Cassandra
 - **Frameworks**: Actix Web, Tokio, Serde

## Docs
[Open API](https://petstore.swagger.io/?url=https://raw.githubusercontent.com/leomonteiro92/mini_ledger_rust/refs/heads/main/openapi.yml)