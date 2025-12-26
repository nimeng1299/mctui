use std::time::Instant;

use mc_core::account::Account;
use ratatui::widgets::TableState;
use serde::{Deserialize, Serialize};
use tui_input::Input;

pub struct AccountData {
    pub show_mode: AccountShowMode,
    pub tabs_keys_instant: Instant,
    pub is_input_mode: bool,
    pub seleted_account_index: usize,
    pub table_state: TableState,

    // add offline
    pub add_offline_account_focus: AddOfflineAccountFocus,
    pub add_offline_username_input: Input,
}

impl AccountData {}

impl Default for AccountData {
    fn default() -> Self {
        Self {
            show_mode: AccountShowMode::default(),
            tabs_keys_instant: Instant::now(),
            is_input_mode: false,
            seleted_account_index: 0,
            table_state: TableState::default(),
            add_offline_account_focus: AddOfflineAccountFocus::None,
            add_offline_username_input: Input::default(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum AddOfflineAccountFocus {
    None,
    UserNameInput,
    Add,
}

impl AddOfflineAccountFocus {
    pub fn tab(&self) -> Self {
        match self {
            Self::None => Self::UserNameInput,
            Self::UserNameInput => Self::Add,
            Self::Add => Self::UserNameInput,
        }
    }
}

#[derive(Default)]
pub enum AccountShowMode {
    #[default]
    Table,
    AddOnlineAccount,
    AddOfflineAccount,
    AddOtherAccount,
}

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
