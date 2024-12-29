CREATE KEYSPACE mini_ledger WITH REPLICATION = 
    { 'class' : 'SimpleStrategy', 'replication_factor' : 1 };

CREATE TABLE mini_ledger.accounts (
    uuid UUID PRIMARY KEY,
    currency TEXT,
    balance DECIMAL,
    created_at_in_nanos BIGINT,
    last_updated_at_in_nanos BIGINT,
    version UUID
);

CREATE TABLE mini_ledger.transactions_by_account_time_range (
    idempotency_key UUID,
    uuid UUID,
    account_uuid UUID,
    amount DECIMAL,
    created_at_in_nanos BIGINT,
    currency TEXT,
    PRIMARY KEY (account_uuid, created_at_in_nanos, uuid)
) WITH CLUSTERING ORDER BY (created_at_in_nanos DESC);

CREATE TABLE mini_ledger.transactions_by_account (
    account_uuid UUID,
    created_at_in_nanos BIGINT,
    idempotency_key UUID,
    uuid UUID,
    amount DECIMAL,
    currency TEXT,
    PRIMARY KEY (account_uuid, created_at_in_nanos, uuid)
) WITH CLUSTERING ORDER BY (created_at_in_nanos DESC);