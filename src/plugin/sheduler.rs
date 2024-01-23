use std::time::Instant;

use super::source::Source;

#[derive(Default)]
pub struct Scheduler {
    sources: Vec<Source>,
}

impl Scheduler {
    pub fn new(sources: Vec<Source>) -> Self {
        Self { sources }
    }

    pub async fn spawn(&mut self) {
        loop {
            let mut next_run = None;

            for source in &mut self.sources {
                if source.should_run() {
                    source.run().await;
                }
                next_run = Some(
                    next_run
                        .map(|nr: Instant| nr.min(source.next_run))
                        .unwrap_or(source.next_run),
                );
            }

            if let Some(nr) = next_run {
                let now = Instant::now();
                if nr > now {
                    tokio::time::sleep(nr - now).await;
                }
            }
        }
    }
}

