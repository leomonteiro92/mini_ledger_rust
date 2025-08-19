use base::model::{Account, Transaction};
use std::sync::Arc;

use async_trait::async_trait;
use bigdecimal::BigDecimal;
use cassandra_cpp::{AsRustType, BatchType, BindRustType, Session};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CassandraStorage {
    session: Arc<Session>,
}

impl CassandraStorage {
    pub fn new(session: Arc<Session>) -> Self {
        CassandraStorage { session }
    }
}

#[async_trait]
impl base::storage::Storage for CassandraStorage {
    async fn save_account(&self, account: Account) -> Result<(), String> {
        let mut stmt = self.session.statement(
            r#"INSERT INTO mini_ledger.accounts
                (id, currency, balance, created_at_in_nanos,
                last_updated_at_in_nanos, version)
                VALUES (?, ?, ?, ?, ?, ?) IF NOT EXISTS"#,
        );
        stmt.bind(0, account.uuid).unwrap();
        stmt.bind(1, account.currency.as_str()).unwrap();
        stmt.bind(2, account.balance.with_scale(2).to_string().as_str())
            .unwrap();
        stmt.bind(3, account.created_at.timestamp_nanos_opt().unwrap())
            .unwrap();
        stmt.bind(4, account.last_updated_at.timestamp_nanos_opt().unwrap())
            .unwrap();
        stmt.bind(5, Uuid::new_v4()).unwrap();
        stmt.execute().await.unwrap();
        Ok(())
    }

    async fn get_account(&self, uuid: Uuid) -> Result<Option<Account>, String> {
        let query = r#"SELECT id, currency, balance, created_at_in_nanos,
            last_updated_at_in_nanos, version FROM mini_ledger.accounts WHERE id = ?
            "#;
        let mut stmt = self.session.statement(query);
        stmt.bind(0, uuid).unwrap();
        let rows = stmt.execute().await.unwrap();

        match rows.first_row() {
            None => return Ok(None),
            Some(row) => {
                let balance_as_str: String = row.get_by_name("balance").unwrap();
                let balance = balance_as_str.parse::<BigDecimal>().unwrap().with_scale(2);
                Ok(Some(Account::from_storage(
                    row.get_by_name("id").unwrap(),
                    row.get_by_name("currency").unwrap(),
                    balance,
                    row.get_by_name("created_at_in_nanos").unwrap(),
                    row.get_by_name("last_updated_at_in_nanos").unwrap(),
                    row.get_by_name("version").unwrap(),
                )))
            }
        }
    }

    async fn save_transactions(
        &self,
        created_transactions: Vec<Transaction>,
        updated_accounts: Vec<Account>,
    ) -> Result<Vec<Transaction>, String> {
        let mut changes = self.session.batch(BatchType::LOGGED);

        let mut txs: Vec<Transaction> = Vec::new();
        for transaction in created_transactions {
            let mut stmt = self.session.statement(
                r#"INSERT INTO mini_ledger.transactions_by_account_time_range
                    (idempotency_key, id, account_id, amount, created_at_in_nanos, currency)
                    VALUES (?, ?, ?, ?, ?, ?) IF NOT EXISTS;"#,
            );
            stmt.bind(0, transaction.idempotency_key.as_str()).unwrap();
            stmt.bind(1, transaction.id).unwrap();
            stmt.bind(2, transaction.account_id).unwrap();
            stmt.bind(3, transaction.amount.with_scale(2).to_string().as_str())
                .unwrap();
            stmt.bind(4, transaction.created_at.timestamp_nanos_opt().unwrap())
                .unwrap();
            stmt.bind(5, transaction.currency.as_str()).unwrap();
            changes.add_statement(stmt).unwrap();
            txs.push(transaction);
        }

        for account in updated_accounts {
            let mut stmt = self.session.statement(
                r#"UPDATE mini_ledger.accounts
                    SET balance = ?, last_updated_at_in_nanos = ?, version = ?
                    WHERE id = ? IF version = ?;"#,
            );
            stmt.bind(0, account.balance.with_scale(2).to_string().as_str())
                .unwrap();
            stmt.bind(1, account.last_updated_at.timestamp_nanos_opt().unwrap())
                .unwrap();
            stmt.bind(2, Uuid::new_v4()).unwrap();
            stmt.bind(3, account.uuid).unwrap();
            stmt.bind(4, account.version).unwrap();
            changes.add_statement(stmt).unwrap();
        }

        changes.execute().await.unwrap();
        Ok(txs)
    }
}
