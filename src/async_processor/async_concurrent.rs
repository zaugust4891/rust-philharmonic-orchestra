use tokio::sync::mpsc as async_mpsc;
use tokio::task;
use tokio::time;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug)]

pub struct AsyncJob {
	pub id: usize,
	pub data: String,
	pub priority: u8 // 0 = highest priority
}

pub struct AsyncOrchestrator {
	pub job_sender: async_mpsc::Sender<AsyncJob>,
}

impl AsyncOrchestrator {
	pub fn new(worker_count: usize) -> Self {
		let (tx, mut rx) = async_mpsc::channel::<AsyncJob>(100);
		task::spawn(async move {
			let mut job_queue: Vec<AsyncJob> = Vec::new();
			let semaphore = Arc::new(tokio::sync::Semaphore::new(worker_count));
			while let Some(job) = rx.recv().await {
				job_queue.push(job);
				job_queue.sort_by_key(|j| j.priority);

				while let Some(job) = job_queue.pop() {
					let permit = semaphore.clone().acquire_owned().await.unwrap();

					task::spawn(async move {
						println!("Processing job {} (priority {}): {}", job.id, job.priority, job.data);

						time::sleep(Duration::from_millis(100)).await;

						println!("Completed job {}", job.id);
						drop(permit);

					});

				}
			}
		});
		AsyncOrchestrator { job_sender: tx }
	}

	pub async fn submit_job(&self, job: AsyncJob) {
		self.job_sender.send(job).await.unwrap();
	}
}
	