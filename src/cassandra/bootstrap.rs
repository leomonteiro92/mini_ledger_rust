use cassandra_cpp::Session;

pub async fn bootstrap(session: &Session) -> Result<(), String> {
    session
        .execute("DROP KEYSPACE IF EXISTS mini_ledger;")
        .await
        .unwrap();
    session
        .execute(
            r#"
        CREATE KEYSPACE IF NOT EXISTS mini_ledger WITH REPLICATION = 
    { 'class' : 'SimpleStrategy', 'replication_factor' : 1 };
    "#,
        )
        .await
        .unwrap();
    session
        .execute(
            r#"
        CREATE TABLE IF NOT EXISTS mini_ledger.accounts (
            id UUID PRIMARY KEY,
            currency TEXT,
            balance DECIMAL,
            created_at_in_nanos BIGINT,
            last_updated_at_in_nanos BIGINT,
            version UUID
        );
    "#,
        )
        .await
        .unwrap();
    session
        .execute(
            r#"CREATE TABLE IF NOT EXISTS mini_ledger.transactions_by_account_time_range (
                idempotency_key TEXT,
                id UUID,
                account_id UUID,
                amount DECIMAL,
                created_at_in_nanos BIGINT,
                currency TEXT,
                PRIMARY KEY (account_id, created_at_in_nanos)
            ) WITH CLUSTERING ORDER BY (created_at_in_nanos DESC);"#,
        )
        .await
        .unwrap();
    // ...
    Ok(())
}
