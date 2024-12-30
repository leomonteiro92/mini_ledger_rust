use uuid::Uuid;

use crate::model::Account;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccountCreationDTO {
    uuid: Uuid,
    currency: String,
}

impl AccountCreationDTO {
    pub fn to_account(&self) -> Account {
        Account::new(self.uuid, &self.currency)
    }
}
