use uuid::Uuid;

use crate::model::account::Account;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccountCreationRequest {
    uuid: Uuid,
    currency: String,
}

impl AccountCreationRequest {
    pub fn to_account(&self) -> Account {
        Account::new(self.uuid, &self.currency)
    }
}
