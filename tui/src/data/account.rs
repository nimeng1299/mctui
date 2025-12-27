use mc_core::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccountSetting {
    pub current_account: Option<Account>,
    pub all_accounts: Vec<Account>,
}

impl Default for AccountSetting {
    fn default() -> Self {
        Self {
            current_account: None,
            all_accounts: Vec::new(),
        }
    }
}