use std::sync::Arc;

use app_core::service::{AccountService, TransactionService};

pub struct AppState {
    pub account_service: Arc<dyn AccountService>,
    pub transaction_service: Arc<dyn TransactionService>,
}

impl AppState {
    pub fn new(
        account_service: Arc<dyn AccountService>,
        transaction_service: Arc<dyn TransactionService>,
    ) -> Self {
        AppState {
            account_service,
            transaction_service,
        }
    }
}
