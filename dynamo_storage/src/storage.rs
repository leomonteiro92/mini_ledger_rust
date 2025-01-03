use std::sync::Arc;

use async_trait::async_trait;
use aws_sdk_dynamodb::{
    types::{AttributeValue, Put, TransactWriteItem, Update},
    Client,
};
use base::{
    model::{Account, Transaction},
    storage::Storage,
};
use uuid::Uuid;

use crate::entity::AccountEntity;

const TABLE_NAME: &str = "mini_ledger";

#[derive(Debug, Clone)]
pub struct DynamoStorage {
    client: Arc<Client>,
}

impl DynamoStorage {
    pub fn new(client: Arc<Client>) -> Self {
        DynamoStorage { client }
    }

    fn create_attr_value<T: ToString>(value: T) -> AttributeValue {
        AttributeValue::S(value.to_string())
    }

    fn create_number_attr(value: impl ToString) -> AttributeValue {
        AttributeValue::N(value.to_string())
    }

    fn format_pk(prefix: &str, id: &Uuid) -> String {
        format!("{}{}", prefix, id)
    }
}

#[async_trait]
impl Storage for DynamoStorage {
    async fn save_account(&self, account: Account) -> Result<(), String> {
        let pk = Self::format_pk("acc#", &account.uuid);
        let request = self
            .client
            .put_item()
            .table_name(TABLE_NAME)
            .item("pk", Self::create_attr_value(&pk))
            .item("sk", Self::create_attr_value(&pk))
            .item("uuid", Self::create_attr_value(account.uuid))
            .item("currency", Self::create_attr_value(account.currency))
            .item("balance", Self::create_number_attr(account.balance))
            .item(
                "created_at_in_millis",
                Self::create_number_attr(account.created_at.timestamp_millis()),
            )
            .item(
                "last_updated_at_in_millis",
                Self::create_number_attr(account.last_updated_at.timestamp_millis()),
            )
            .item("version", Self::create_attr_value(account.version));

        request
            .send()
            .await
            .map_err(|e| format!("Failed to save account: {:?}", e))?;

        Ok(())
    }

    async fn get_account(&self, uuid: Uuid) -> Result<Option<Account>, String> {
        let pk = Self::format_pk("acc#", &uuid);

        let results = self
            .client
            .query()
            .table_name(TABLE_NAME)
            .key_condition_expression("#pk = :pk")
            .expression_attribute_names("#pk", "pk")
            .expression_attribute_values(":pk", AttributeValue::S(pk))
            .send()
            .await
            .map_err(|e| format!("Failed to get account: {:?}", e))?;

        if let Some(items) = results.items {
            let accounts: Vec<AccountEntity> = items.iter().map(|v| v.into()).collect();
            let account: Account = accounts.first().ok_or("Account not found")?.into();
            Ok(Some(account))
        } else {
            Ok(None)
        }
    }

    async fn save_transactions(
        &self,
        created_transactions: Vec<Transaction>,
        updated_accounts: Vec<Account>,
    ) -> Result<Vec<Transaction>, String> {
        let mut builder = self.client.transact_write_items();

        for tx in &created_transactions {
            let pk = Self::format_pk("tx#acc#", &tx.account_id);
            let sk = Self::format_pk("tx#", &tx.id);
            let put = Put::builder()
                .table_name(TABLE_NAME)
                .item("pk", AttributeValue::S(pk))
                .item("sk", AttributeValue::S(sk))
                .item("account_id", Self::create_attr_value(tx.account_id))
                .item(
                    "account_version",
                    Self::create_attr_value(tx.account_version),
                )
                .item("amount", Self::create_number_attr(tx.amount.with_prec(2)))
                .item(
                    "created_at_in_millis",
                    Self::create_number_attr(tx.created_at.timestamp_millis()),
                )
                .item("currency", Self::create_attr_value(&tx.currency))
                .item("id", Self::create_attr_value(tx.id))
                .item(
                    "idempotency_key",
                    Self::create_attr_value(&tx.idempotency_key),
                )
                .build()
                .map_err(|e| format!("Failed to build put expression: {:?}", e))?;
            let op = TransactWriteItem::builder().put(put).build();
            builder = builder.transact_items(op);
        }

        for acc in &updated_accounts {
            let pk = Self::format_pk("acc#", &acc.uuid);
            let new_version = Uuid::new_v4();
            let update = Update::builder()
                .table_name(TABLE_NAME)
                .key("pk", Self::create_attr_value(&pk))
                .key("sk", Self::create_attr_value(&pk))
                .update_expression("SET balance = :amount, version = :newVersion")
                .condition_expression("version = :expectedVersion")
                .expression_attribute_values(":amount", Self::create_number_attr(&acc.balance))
                .expression_attribute_values(":newVersion", Self::create_attr_value(new_version))
                .expression_attribute_values(
                    ":expectedVersion",
                    Self::create_attr_value(acc.version),
                )
                .build()
                .map_err(|e| format!("Failed to build update expression: {:?}", e))?;
            let op = TransactWriteItem::builder().update(update).build();
            builder = builder.transact_items(op);
        }

        builder
            .send()
            .await
            .map_err(|e| format!("Failed to save transactions: {:?}", e))?;

        Ok(created_transactions)
    }
}
