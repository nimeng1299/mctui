//! Data struct for the application
pub mod account;
pub mod download;



use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Error, Result};
use directories::ProjectDirs;
use log::{info, warn};
use rat_salsa::{SalsaAppContext, SalsaContext};
use rat_theme4::{create_salsa_theme, theme::SalsaTheme};
use rat_widget::menu::MenuLineState;
use serde::{Deserialize, Serialize};

use crate::{data::account::AccountSetting, event::AppEvent};

/// This module defines the data structure used to hold the state of the application.
pub struct AppData {
    pub menu_selected: MenuLineState,
    pub download_data: download::DownloadData,
}

impl AppData {}

impl Default for AppData {
    fn default() -> Self {
        let mut menu_selected = MenuLineState::default();
        menu_selected.select(Some(0));
        Self {
            menu_selected: menu_selected,
            download_data: download::DownloadData::default(),
        }
    }
}

/// This module defines the data structure used to hold the settings of the application.
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub mspt: u64,              // milliseconds per tick event
    pub download_thread: usize, // download threads
    pub theme_name: String,
    // account settings
    pub account_setting: AccountSetting,
    #[serde(skip)]
    pub theme: SalsaTheme,
    #[serde(skip)]
    pub ctx: SalsaAppContext<AppEvent, Error>,
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
                setting.theme = create_salsa_theme(&setting.theme_name);
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

impl SalsaContext<AppEvent, Error> for Settings {
    fn set_salsa_ctx(&mut self, app_ctx: rat_salsa::SalsaAppContext<AppEvent, Error>) {
        self.ctx = app_ctx;
    }

    fn salsa_ctx(&self) -> &SalsaAppContext<AppEvent, Error> {
        &self.ctx
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            mspt: 10,
            download_thread: 8,
            theme_name: "Reds Shell".to_string(),
            account_setting: AccountSetting::default(),
            theme: create_salsa_theme("Reds Shell"),
            ctx: SalsaAppContext::default(),
        }
    }
}
