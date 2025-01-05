use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::Account;

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountCreationDTO {
    pub uuid: Uuid,
    pub currency: String,
}

impl Into<Account> for AccountCreationDTO {
    fn into(self) -> Account {
        Account::new(self.uuid, &self.currency)
    }
}
