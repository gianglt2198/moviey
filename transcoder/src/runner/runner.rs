use std::{sync::Arc, time::Duration};

use notify_debouncer_full::{DebounceEventResult, new_debouncer, notify::RecursiveMode};
use tokio::sync::mpsc;

use crate::runner::handle_event_async;

#[derive(Clone, Debug)]
pub struct Runner {
    pool: Arc<sqlx::PgPool>,
    input_dir: String,
    output_dir: String,
}

impl Runner {
    pub fn new(pool: Arc<sqlx::PgPool>, input_dir: String, output_dir: String) -> Self {
        Runner {
            pool,
            input_dir,
            output_dir,
        }
    }

    pub async fn start(&self) {
        let (tx, mut rx) = mpsc::channel::<DebounceEventResult>(100);

        let input_dir = self.input_dir.clone();
        let output_dir = self.output_dir.clone();
        let pool = self.pool.clone();

        let watcher_handle = tokio::task::spawn_blocking(move || {
            // 1. Initialize the Debouncer (2-second delay)
            // This waits for the file to "stop changing" before firing the event
            let mut debouncer = new_debouncer(
                Duration::from_secs(2),
                None,
                move |result: DebounceEventResult| {
                    if let Err(e) = tx.blocking_send(result) {
                        eprintln!("Failed to send event to async task: {:?}", e);
                    }
                },
            )
            .unwrap();

            // 2. Watch the input directory
            debouncer
                .watch(input_dir.clone(), RecursiveMode::NonRecursive)
                .expect("Failed to start watcher");

            println!("🚀 System active. Watching {}...", input_dir);

            // The thread needs to stay alive to process events. A loop is typically used here,
            // but for this simple example, we rely on the main function holding the runtime open.
            // In a real application, you'd manage the lifecycle more robustly.
            // The `debouncer` guard will stop the watcher when it's dropped.
            // To keep it running, one option is to use a channel to signal when to stop the loop.
            // We can use a trick to keep the debouncer in scope:
            loop {
                std::thread::sleep(Duration::from_secs(1));
            }
        });

        while let Some(result) = rx.recv().await {
            match result {
                Ok(events) => {
                    for event in events {
                        let p = pool.clone().as_ref().clone();
                        let out = output_dir.clone();
                        tokio::spawn(async move {
                            handle_event_async(p, event, out).await;
                        });
                    }
                }
                Err(errors) => errors
                    .iter()
                    .for_each(|e| eprintln!("Watch error: {:?}", e)),
            }
        }

        // Keep main alive without blocking the thread
        watcher_handle.await.unwrap();
    }
}
