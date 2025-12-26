use crate::account::base::AccountBase;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct OfflineAccount {
    pub username: String,
    pub uuid: String,
}

impl OfflineAccount {
    pub fn new(username: String) -> Self {
        let uuid = format!("{:x}", md5::compute(&username));
        OfflineAccount { username, uuid }
    }

    pub fn new_with_uuid(username: String, uuid: String) -> Self {
        OfflineAccount { username, uuid }
    }
}

impl AccountBase for OfflineAccount {
    const KIND: &'static str = "offline";
    fn get_username(&self) -> &str {
        &self.username
    }
    fn get_access_token(&self) -> &str {
        &self.uuid
    }
    fn get_uuid(&self) -> &str {
        // MD5: OfflinePlayer: + username
        &self.uuid
    }
    fn get_user_type(&self) -> &str {
        "msa"
    }
}
