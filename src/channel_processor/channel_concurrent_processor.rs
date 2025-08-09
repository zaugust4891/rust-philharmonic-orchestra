use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Job {
    pub id: usize,
    pub data: String,
}
pub enum OrchestratorMessage {
	NewJob(Job),
	Shutdown,
}

pub struct Orchestrator {
	pub workers: Vec<Worker>,
	pub sender: Sender<OrchestratorMessage>,
}

pub struct Worker {
	pub id: usize,
	pub thread: Option<thread::JoinHandle<()>>,
}

impl Orchestrator {
	pub fn new(worker_count: usize) -> Self {
		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));
		let mut workers = Vec::with_capacity(worker_count);

		for id in 0..worker_count {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		Orchestrator { workers, sender }
	}

	pub fn submit_job(&self, job: Job) {
		self.sender.send(OrchestratorMessage::NewJob(job)).unwrap();
	}

	pub fn shutdown(mut self) {
		println!("\nShutting down orchestrator...");
		for _ in &self.workers {
			self.sender.send(OrchestratorMessage::Shutdown).unwrap();
		}

		for worker in &mut self.workers {
			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
				println!("Worker {} shut down.", worker.id);
			}
		}
	}
}

impl Worker {
	pub fn new(id: usize, receiver: Arc<Mutex<Receiver<OrchestratorMessage>>>) -> Self {
		let thread = thread::spawn(move || {
			loop {
				let message = receiver.lock().unwrap().recv().unwrap();
				match message {
					OrchestratorMessage::NewJob(job) => {
						println!("Worker {} received job {}: {}", id, job.id, job.data);
						let processing_time = if job.data.contains("priority") {
							50
						} else {
							150
						};
						thread::sleep(Duration::from_millis(processing_time));
						println!("Worker {} completed job {}", id, job.id);
					}
					OrchestratorMessage::Shutdown => {
						println!("Worker {} received shutdown signal.", id);
						break;
					}
				}
			}
		});

		Worker { 
			id,
			thread: Some(thread)
		}
	}
}