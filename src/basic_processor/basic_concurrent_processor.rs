use std::sync::{Arc, Mutex};
use std::thread::{self, spawn};
use std::time::Duration;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Job {
	id: usize,
	data: String,

}

pub fn simple_concurrent_processor() {
	let jobs: Vec<Job> = (0..10)
		.map(|i| Job {
			id: i,
			data: format!("Processing job {}", i),
		})
		.collect();


	let job_queue = Arc::new(Mutex::new(VecDeque::from(jobs)));
	let mut handles = vec![];

	for worker_id in 0..3 {
		let queue = Arc::clone(&job_queue);
		let handle = spawn(move || {
			loop {
				let job = {
					let mut q = queue.lock().unwrap();
					q.pop_front()
				};

				match job {
					Some(job) => {
						println!("Worker {} Processing Job {}: {}", 
							worker_id, job.id, job.data);
						thread::sleep(Duration::from_millis(100));
						println!("Worker {} completed job {}", worker_id, job.id);
					}

					None => {
						println!("Worker {} finished. No more jobs to complete.", worker_id);
						break;
					}
				}
			}
		});

		handles.push(handle)
	}

	for handle in handles {
		handle.join().unwrap();
	}
}


