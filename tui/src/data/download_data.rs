use std::time::Instant;

use mc_core::download::download_pool::DownloadPool;
use tui_input::Input;

pub struct DownloadData {
    pub tabs_list_state: usize,
    pub tabs_keys_instant: Instant, // 用于防止快速切换标签页
    pub download_pool: DownloadPool,
    pub is_input_mode: bool,
    // debug tab
    pub debug_focus: DebugFocus,
    pub debug_keys_instant: Instant,
    pub debug_enter_instant: Instant,
    pub debug_input_url: Input,
    pub debug_input_path: Input,
}

impl DownloadData {
    pub fn change_tabs_list_state(&mut self, index: usize) {
        self.tabs_list_state = index;

        self.debug_focus = DebugFocus::None;
    }
}

impl Default for DownloadData {
    fn default() -> Self {
        Self {
            tabs_list_state: 0,
            tabs_keys_instant: Instant::now(),
            download_pool: DownloadPool::new(4),
            is_input_mode: false,
            debug_focus: DebugFocus::None,
            debug_keys_instant: Instant::now(),
            debug_enter_instant: Instant::now(),
            debug_input_url: Input::default(),
            debug_input_path: Input::default(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum DebugFocus {
    None,
    InputUrl,
    InputPath,
    SelectPath,
    Download,
}

impl DebugFocus {
    pub fn tab(&self) -> Self {
        match self {
            Self::None => Self::InputUrl,
            Self::InputUrl => Self::InputPath,
            Self::InputPath => Self::SelectPath,
            Self::SelectPath => Self::Download,
            Self::Download => Self::None,
        }
    }
}
