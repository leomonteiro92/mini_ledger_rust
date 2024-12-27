use std::sync::Arc;

use crate::service::types::AccountService;

pub struct AppState {
    pub account_service: Arc<dyn AccountService>,
}

impl AppState {
    pub fn new(account_service: Arc<dyn AccountService>) -> Self {
        AppState { account_service }
    }
}
