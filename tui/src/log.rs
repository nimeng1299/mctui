use anyhow::{Context, Result};

pub fn init_log() -> Result<()> {
    std::fs::create_dir_all("log").context("create log directory failed")?;
    let now = chrono::Local::now();
    let filename = now.format("mctui_%Y-%m-%d_%H-%M-%S.log");
    let dir = std::path::Path::new("log").join(filename.to_string());

    Ok(())
}
