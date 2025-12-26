//! Data struct for the application
pub mod account_data;
pub mod download_data;
pub mod game_data;

use std::{
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::{Context, Result};
use crossterm::event::{KeyEvent, MouseEvent};
use directories::ProjectDirs;
use log::{info, warn};
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

use crate::data::{
    account_data::{AccountData, AccountSetting},
    download_data::DownloadData,
};
/// This module defines the data structure used to hold the state of the application.
pub struct AppData {
    pub message: String,
    pub is_quit: bool,
    pub is_read_event: bool,

    pub menu_list_state: ListState, // 菜单栏
    pub menu_list_keys_instant: Instant,

    pub keys_instant: Instant,
    pub mouse_instant: Instant,
    // ui.account
    pub account_data: AccountData,
    // ui.game
    pub game_data: game_data::GameData,
    // ui.download
    pub download_data: DownloadData,

    pub event: Option<crossterm::event::Event>,
    pub key_event: Option<KeyEvent>,
    pub mouse_event: Option<MouseEvent>,
}

impl AppData {
    pub fn update_from_setting(&mut self, setting: &Settings) {
        self.download_data
            .download_pool
            .change_max_workers(setting.download_thread);
    }
}

impl Default for AppData {
    fn default() -> Self {
        let mut menu_list_state = ListState::default();
        menu_list_state.select_first();
        Self {
            message: String::new(),
            is_quit: false,
            is_read_event: false,
            menu_list_state,
            menu_list_keys_instant: Instant::now(),
            keys_instant: Instant::now(),
            mouse_instant: Instant::now(),
            account_data: AccountData::default(),
            game_data: game_data::GameData::default(),
            download_data: DownloadData::default(),
            event: None,
            key_event: None,
            mouse_event: None,
        }
    }
}

/// This module defines the data structure used to hold the settings of the application.
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub mspt: u64,              // milliseconds per tick event
    pub download_thread: usize, // download threads
    // account settings
    pub account_setting: AccountSetting,
}

impl Settings {
    pub fn load(path: &Path) -> Result<Self> {
        fs::create_dir_all(path)
            .context(format!("cannot create settings directory: {:?}", path))?;
        let file = path.join("settings.toml");
        let mut setting = Self::default();

        if file.is_file() {
            if let Ok(content) = fs::read_to_string(file) {
                setting = toml::from_str(&content).context("failed to parse settings file")?;
                info!(target:"MCTui", "Settings loaded.");
            } else {
                warn!(target:"MCTui", "Failed to read settings file, using default settings.");
            }
        } else {
            warn!(target:"MCTui", "Settings file not found, using default settings.");
        }
        return Ok(setting);
    }

    pub fn load_default() -> Result<Self> {
        if let Some(proj_dirs) = ProjectDirs::from_path(PathBuf::from("mctui")) {
            return Self::load(&proj_dirs.config_dir());
        };
        Ok(Self::default())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path)
            .context(format!("cannot create settings directory: {:?}", path))?;
        let file = path.join("settings.toml");
        let content = toml::to_string_pretty(self).context("failed to serialize settings")?;
        fs::write(&file, content)
            .context(format!("failed to write settings to file: {:?}", file))?;
        info!(target:"MCTui", "Settings saved.");
        Ok(())
    }

    pub fn save_default(&self) -> Result<()> {
        if let Some(proj_dirs) = ProjectDirs::from_path(PathBuf::from("mctui")) {
            return self.save(&proj_dirs.config_dir());
        };
        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            mspt: 10,
            download_thread: 8,
            account_setting: AccountSetting::default(),
        }
    }
}
