use crate::{download::single_downloader::{SingleDownloader, SingleDownloaderEvent}, install::minecraft::version_manifest::MinecraftVersionManifest, statue::Status};

pub mod version_manifest;

/// 从Mojang获取所有Minecraft版本
pub fn get_all_minecraft_versions(downloader: &mut SingleDownloader) -> Status<MinecraftVersionManifest, (), String> {
    get_all_minecraft_versions_from_url(
        "https://launchermeta.mojang.com/mc/game/version_manifest.json".to_string(),
        downloader,
    )
}

/// 从指定URL获取所有Minecraft版本
pub fn get_all_minecraft_versions_from_url(url: String, downloader: &mut SingleDownloader) -> Status<MinecraftVersionManifest, (), String> {
    match downloader.get_state() {
        SingleDownloaderEvent::None => {
            downloader.download(url);
            Status::Progress(())
        }
        SingleDownloaderEvent::Progress => {
            Status::Progress(())
        }
        SingleDownloaderEvent::Finished => {
            let data = downloader.get_data();
            match serde_json::from_str::<MinecraftVersionManifest>(&data) {
                Ok(manifest) => Status::Success(manifest),
                Err(e) => Status::Failed(e.to_string()),
            }
        }
        SingleDownloaderEvent::Failed => {
            Status::Failed(downloader.get_error())
        }
    }
}