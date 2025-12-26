use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
    },
    thread,
    time::{Duration, Instant},
};

use crate::download::download_event::DownloadEvent;

pub(super) fn spawn_download_worker(
    id: usize,
    sender: Sender<DownloadEvent>,
    stop_flag: Arc<AtomicBool>,
    url: String,
    save_path: PathBuf,
) {
    thread::spawn(move || {
        let mut downloaded_size = 0u64;
        if let Ok(mut response) = attohttpc::get(url.clone()).send() {
            let buffer_size = 16 * 1024; // 16KB
            let mut buffer = vec![0u8; buffer_size];

            let mut file = match File::create(&save_path) {
                Ok(f) => f,
                Err(e) => {
                    let _ = sender.send(DownloadEvent::FailTask {
                        id,
                        error: format!("Failed to create file: {:?}", e),
                    });
                    return;
                }
            };

            let total_size = response
                .headers()
                .get("Content-Length")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());

            if let Some(len) = total_size {
                let _ = sender.send(DownloadEvent::FileContent { id, len });
            }

            let mut failed_count = 0;
            let max_failed_count = 3;

            let mut last_downloaded_size = 0u64;
            let mut last_tick = Instant::now();
            loop {
                if stop_flag.load(Ordering::Relaxed) {
                    if failed_count >= max_failed_count {
                        let _ = sender.send(DownloadEvent::FailTask {
                            id,
                            error: "Maximum retry attempts reached".to_string(),
                        });
                        break;
                    } else {
                        failed_count += 1;
                        continue;
                    }
                }

                let bytes_read = response.read(&mut buffer).unwrap_or(0);
                if bytes_read == 0 {
                    let _ = sender.send(DownloadEvent::Finished { id });
                    break;
                }

                let _ = file.write_all(&buffer[..bytes_read]);

                downloaded_size += bytes_read as u64;
                last_downloaded_size += bytes_read as u64;

                if last_tick.elapsed() >= Duration::from_secs(1) {
                    let speed = last_downloaded_size / last_tick.elapsed().as_secs();
                    let _ = sender.send(DownloadEvent::Progress {
                        id,
                        downloaded_size,
                        speed,
                    });

                    last_tick = Instant::now();
                    last_downloaded_size = 0;
                }
                thread::sleep(Duration::from_millis(10));
            }
        } else {
            let _ = sender.send(DownloadEvent::FailTask {
                id,
                error: format!("Failed to download from URL: {}", url),
            });
        }
    });
}
