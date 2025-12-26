use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc::{self, Sender},
    },
    thread,
};

use log::{error, info};

use crate::download::{
    download_event::{DownloadEvent, DownloadFinished, DownloadStatus, DownloadTask},
    download_url::spawn_download_worker,
};

pub struct DownloadPool {
    sender: Sender<DownloadEvent>,
    max_workers: Arc<AtomicUsize>,
    pub have_failed: Arc<AtomicBool>,
}

impl DownloadPool {
    pub fn new(max_workers: usize) -> Self {
        let (tx, rx) = mpsc::channel::<DownloadEvent>();
        let actor_tx = tx.clone();

        // 全局停止标志（worker 共享）
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_actor = stop_flag.clone();

        let max_workers = Arc::new(AtomicUsize::new(max_workers));
        let max_workers_actor = max_workers.clone();

        let have_failed = Arc::new(AtomicBool::new(false));
        let have_failed_actor = have_failed.clone();

        thread::spawn(move || {
            let mut tasks: Vec<DownloadTask> = Vec::new();
            let mut queue: VecDeque<usize> = VecDeque::new();
            let mut running = 0usize;
            let mut stopping = false;

            for cmd in rx {
                match cmd {
                    DownloadEvent::AddTask((url, save_path)) => {
                        if !stopping {
                            let id = tasks.len();
                            tasks.push(DownloadTask {
                                progress: 0f64,
                                downloaded_size: 0,
                                file_len: None,
                                speed: 0,
                                finished: DownloadFinished::Progress,
                                url: url,
                                save_path: save_path,
                            });
                            queue.push_back(id);
                        }
                    }

                    DownloadEvent::FailTask { id, error } => {
                        if let Some(task) = tasks.get_mut(id) {
                            task.finished = DownloadFinished::Failed;
                            task.speed = 0;
                            running = running.saturating_sub(1);
                            error!(target: "download_core", "{} download failed: {}", task.url, error);
                            have_failed_actor.store(true, Ordering::Relaxed);
                        }
                    }

                    DownloadEvent::FileContent { id, len } => {
                        if let Some(task) = tasks.get_mut(id) {
                            task.file_len = Some(len);
                        }
                    }

                    DownloadEvent::Progress {
                        id,
                        downloaded_size,
                        speed,
                    } => {
                        if let Some(task) = tasks.get_mut(id) {
                            if !matches!(
                                task.finished,
                                DownloadFinished::Finished | DownloadFinished::Failed
                            ) && !stopping
                            {
                                if let Some(file_len) = task.file_len {
                                    let delta = (downloaded_size * 100 / file_len) as f64;
                                    task.progress = delta.min(100.0);
                                } else {
                                    // 文件总大小未知
                                    task.progress = 1.0;
                                }
                                task.downloaded_size = downloaded_size;
                                task.speed = speed;
                            }
                        }
                    }

                    DownloadEvent::Finished { id } => {
                        if let Some(task) = tasks.get_mut(id) {
                            if !matches!(
                                task.finished,
                                DownloadFinished::Finished | DownloadFinished::Failed
                            ) {
                                task.finished = DownloadFinished::Finished;
                                task.speed = 0;
                                task.progress = 100.0;
                                running = running.saturating_sub(1);
                                info!(target: "download_core", "download finished: {}", task.url);
                            }
                        }
                    }

                    DownloadEvent::StopAll => {
                        stopping = true;
                        stop_flag_actor.store(true, Ordering::Relaxed);
                        queue.clear(); // 不再调度新任务
                    }

                    DownloadEvent::Query { reply } => {
                        let mut total = 0f64;
                        let mut speed = 0;
                        let per_task = tasks
                            .iter()
                            .map(|t| {
                                total += t.progress;
                                speed += t.speed;
                                (
                                    t.save_path
                                        .file_name()
                                        .and_then(|n| n.to_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    t.progress,
                                    t.downloaded_size,
                                    t.file_len.unwrap_or(0),
                                    t.speed,
                                )
                            })
                            .collect();

                        let _ = reply.send(DownloadStatus {
                            per_task,
                            total,
                            speed,
                            all_total: (tasks.len() as f64) * 100.0,
                            stopping,
                        });

                        //全部下载完成后，清理任务
                        if total == (tasks.len() as f64) * 100.0 && total != 0.0 {
                            tasks.clear();
                            queue.clear();
                        }
                    }
                }

                // 只有在未停止时才调度
                while !stopping && running < max_workers_actor.load(Ordering::Relaxed) {
                    if let Some(id) = queue.pop_front() {
                        running += 1;
                        spawn_download_worker(
                            id,
                            actor_tx.clone(),
                            stop_flag.clone(),
                            tasks[id].url.clone(),
                            tasks[id].save_path.clone(),
                        );
                    } else {
                        break;
                    }
                }
            }
        });

        Self {
            sender: tx,
            max_workers,
            have_failed,
        }
    }

    pub fn add_task(&self, url: String, save_path: String) {
        let _ = self
            .sender
            .send(DownloadEvent::AddTask((url, save_path.into())));
    }

    pub fn stop_all(&self) {
        let _ = self.sender.send(DownloadEvent::StopAll);
    }

    // 查询当前下载状态
    pub fn query(&self) -> DownloadStatus {
        let (tx, rx) = mpsc::channel();
        let _ = self.sender.send(DownloadEvent::Query { reply: tx });
        rx.recv().unwrap()
    }

    pub fn change_max_workers(&self, max_workers: usize) {
        self.max_workers.store(max_workers, Ordering::Relaxed);
    }
}
