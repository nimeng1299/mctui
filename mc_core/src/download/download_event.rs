use std::{path::PathBuf, sync::mpsc::Sender};

pub enum DownloadEvent {
    AddTask((String, PathBuf)),
    FailTask {
        id: usize,
        error: String,
    },
    FileContent {
        id: usize,
        len: u64,
    },
    Progress {
        id: usize,
        downloaded_size: u64,
        speed: u64,
    }, // speed: bytes/s
    Finished {
        id: usize,
    },
    StopAll,
    Query {
        reply: Sender<DownloadStatus>,
    },
}

#[derive(Clone)]
pub struct DownloadStatus {
    pub per_task: Vec<(String, f64, u64, u64, u64)>, // (filename, progress, downloaded_size, file_len, speed)
    pub speed: u64,
    pub total: f64, // 总的下载进度
    pub all_total: f64,// 应该下载的总进度
    pub stopping: bool,
}

#[derive(Clone)]
pub struct DownloadTask {
    pub progress: f64, // 0..=100
    pub downloaded_size: u64,
    pub file_len: Option<u64>,
    pub speed: u64, // bytes per second
    pub finished: DownloadFinished,
    pub url: String,
    pub save_path: PathBuf,
}

#[derive(Clone)]
pub enum DownloadFinished {
    Progress,
    Finished,
    Failed,
}
