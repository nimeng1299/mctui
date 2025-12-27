use mc_core::download::single_downloader::SingleDownloader;
use rat_widget::{menu::MenuLineState, textarea::TextAreaState};

pub struct DownloadData{
    pub download_selected: MenuLineState,
    // minecraft
    pub minecraft_downloader: SingleDownloader,
    pub text_state: TextAreaState,
}


impl Default for DownloadData {
    fn default() -> Self {
        let mut download_selected = MenuLineState::default();
        download_selected.select(Some(0));
        Self {
            download_selected,
            minecraft_downloader: SingleDownloader::default(),
            text_state: TextAreaState::default(),
        }
    }
}