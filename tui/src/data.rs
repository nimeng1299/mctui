//! Data struct for the application
use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use directories::ProjectDirs;
use ratatui::widgets::ListState;
/// This module defines the data structure used to hold the state of the application.
pub struct AppData {
    pub message: String,
    pub is_quit: bool,
    pub menu_list_state: ListState, // 菜单栏
}

impl Default for AppData {
    fn default() -> Self {
        let mut menu_list_state = ListState::default();
        menu_list_state.select_first();
        Self {
            message: String::new(),
            is_quit: false,
            menu_list_state,
        }
    }
}

/// This module defines the data structure used to hold the settings of the application.
pub struct Settings {
    pub mspt: u64, // milliseconds per tick event
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
        Self { mspt: 10 }
    }
}
