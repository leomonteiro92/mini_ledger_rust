use cassandra_cpp::Session;

pub async fn bootstrap(session: &Session) -> Result<(), String> {
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
            uuid TEXT PRIMARY KEY,
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
        .execute("TRUNCATE mini_ledger.accounts")
        .await
        .unwrap();
    // ...
    Ok(())
}
