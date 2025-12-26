use serde::{Deserialize, Serialize};

use crate::account::{base::AccountBase, offline_account::OfflineAccount};

pub mod base;
pub mod offline_account;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Account {
    Offline(OfflineAccount),
}

impl Account {
    pub fn get_type(&self) -> &str {
        match self {
            Account::Offline(_) => OfflineAccount::KIND,
        }
    }
}

impl AccountBase for Account {
    const KIND: &'static str = "account";
    fn get_username(&self) -> &str {
        match self {
            Account::Offline(account) => account.get_username(),
        }
    }
    fn get_access_token(&self) -> &str {
        match self {
            Account::Offline(account) => account.get_access_token(),
        }
    }
    fn get_user_type(&self) -> &str {
        match self {
            Account::Offline(account) => account.get_user_type(),
        }
    }
    fn get_uuid(&self) -> &str {
        match self {
            Account::Offline(account) => account.get_uuid(),
        }
    }
}
