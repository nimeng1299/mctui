//! Data struct for the application
pub mod download_data;
pub mod game_data;
pub mod modpacks_config;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use crossterm::event::{KeyEvent, MouseEvent};
use directories::ProjectDirs;
use ratatui::widgets::ListState;

use crate::data::{download_data::DownloadData, modpacks_config::ModPacksConfig};
/// This module defines the data structure used to hold the state of the application.
pub struct AppData {
    pub message: String,
    pub is_quit: bool,
    pub is_read_event: bool,

    pub menu_list_state: ListState, // 菜单栏
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
            game_data: game_data::GameData::default(),
            download_data: DownloadData::default(),
            event: None,
            key_event: None,
            mouse_event: None,
        }
    }
}

/// This module defines the data structure used to hold the settings of the application.
pub struct Settings {
    pub mspt: u64,              // milliseconds per tick event
    pub download_thread: usize, // download threads
    pub modpacks_config: ModPacksConfig,
}

impl Settings {
    pub fn load(path: &Path) -> Result<Self> {
        fs::create_dir_all(path)
            .context(format!("cannot create settings directory: {:?}", path))?;
        let file = path.join("settings.toml");
        let mut setting = Self::default();

        if file.is_file() {
            if let Ok(content) = fs::read_to_string(file)
                && let Ok(data) = content.parse::<toml::Value>()
                && let Some(tui_value) = data.get("tui")
            {
                if let Some(mspt_value) = tui_value.get("mspt")
                    && let Some(mspt) = mspt_value.as_integer()
                {
                    setting.mspt = mspt as u64;
                }
                if let Some(download_thread_value) = tui_value.get("download_thread")
                    && let Some(download_thread) = download_thread_value.as_integer()
                {
                    setting.download_thread = download_thread as usize;
                }
            }
        }
        return Ok(setting);
    }

    pub fn load_default() -> Result<Self> {
        if let Some(proj_dirs) = ProjectDirs::from_path(PathBuf::from("mctui")) {
            return Self::load(&proj_dirs.config_dir());
        };
        Ok(Self::default())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            mspt: 10,
            download_thread: 8,
            modpacks_config: ModPacksConfig::default(),
        }
    }
}
