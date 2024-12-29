use std::sync::Arc;

use async_trait::async_trait;
use bigdecimal::BigDecimal;
use cassandra_cpp::{AsRustType, BindRustType, Session};
use uuid::Uuid;

use crate::{model::account::Account, storage::Storage};

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
impl Storage for CassandraStorage {
    async fn save_account(&self, account: crate::model::account::Account) -> Result<(), String> {
        let mut stmt = self.session.statement(
            r#"INSERT INTO mini_ledger.accounts 
                (uuid, currency, balance, created_at_in_nanos, 
                last_updated_at_in_nanos, version) 
                VALUES (?, ?, ?, ?, ?, ?)"#,
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

    async fn get_account(
        &self,
        uuid: uuid::Uuid,
    ) -> Result<Option<crate::model::account::Account>, String> {
        let query = r#"SELECT uuid, currency, balance, created_at_in_nanos, 
            last_updated_at_in_nanos, version FROM mini_ledger.accounts WHERE uuid = ?"#;
        let mut stmt = self.session.statement(query);
        stmt.bind(0, uuid).unwrap();
        let rows = stmt.execute().await.unwrap();

        match rows.first_row() {
            None => return Ok(None),
            Some(row) => {
                let balance_as_str: String = row.get_by_name("balance").unwrap();
                let balance = balance_as_str.parse::<BigDecimal>().unwrap().with_scale(2);
                Ok(Some(Account::from_storage(
                    row.get_by_name("uuid").unwrap(),
                    row.get_by_name("currency").unwrap(),
                    balance,
                    row.get_by_name("created_at_in_nanos").unwrap(),
                    row.get_by_name("last_updated_at_in_nanos").unwrap(),
                )))
            }
        }
    }

    async fn save_transactions(
        &mut self,
        _transactions: Vec<crate::model::transaction::Transaction>,
    ) -> Result<Vec<crate::model::transaction::Transaction>, String> {
        Ok(Vec::new())
    }
}
