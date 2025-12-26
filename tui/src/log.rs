use anyhow::{Context, Result};
use tui_logger::{
    TuiLoggerFile, TuiLoggerLevelOutput, init_logger, set_default_level, set_log_file,
};

pub fn init_log() -> Result<()> {
    if cfg!(debug_assertions) {
        init_logger(log::LevelFilter::Debug).context("init logger")?;
    } else {
        init_logger(log::LevelFilter::Info).context("init logger")?;
    }
    set_default_level(log::LevelFilter::Info);

    std::fs::create_dir_all("log").context("create log directory failed")?;
    let now = chrono::Local::now();
    let filename = now.format("mctui_%Y-%m-%d_%H-%M-%S.log");
    let dir = std::path::Path::new("log").join(filename.to_string());
    let file_options = TuiLoggerFile::new(dir.to_str().context("get log file path failed")?)
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .output_file(true)
        .output_separator(':');
    set_log_file(file_options);

    Ok(())
}
