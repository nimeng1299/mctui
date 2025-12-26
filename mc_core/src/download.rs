/*!
 * download from urk
 # example:
 ```
fn main() {
    let pool = DownloadPool::new(4);

    for _ in 0..10 {
        pool.add_task();
    }

    let start = Instant::now();

    loop {
        thread::sleep(Duration::from_secs(1));

        let status = pool.query();

        println!("------------------");
        for (id, progress) in status.per_task.iter().enumerate() {
            println!("Task {} => {}", id, progress);
        }
        println!(
            "Total = {}, stopping = {}",
            status.total, status.stopping
        );

        // can stop after 60s or all tasks finished
        if start.elapsed() >= Duration::from_secs(60) {
            pool.stop_all();
            break;
        }
        if status.total as usize >= status.per_task.len() * 100 {
            break;
        }
    }

    println!("Main thread exiting.");
}
```
 */

pub mod download_event;
pub mod download_pool;
pub mod download_url;
