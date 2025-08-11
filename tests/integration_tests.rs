#[cfg(test)]
mod tests {
    use rust_concurrent_image::basic_processor::basic_concurrent_processor::simple_concurrent_processor;
    use rust_concurrent_image::channel_processor::channel_concurrent_processor::{Job, Orchestrator};
    use rust_concurrent_image::async_processor::async_concurrent::{AsyncJob, AsyncOrchestrator};
    use std::thread;
    use std::time::Duration;
    pub enum Processor {
        Basic,
        Channel,
    }

    pub fn run_channel_processor() {
        println!("\n=== Channel-based Orchestrator ===\n");
    
        let orchestrator = Orchestrator::new(4);
    
        for i in 0..8 {
        let job = Job {
            id: i,
            data: if i % 3 == 0 {
                format!("Process priority task {}", i)
            } else {
                format!("Process regular task {}", i)
            },
        };
        orchestrator.submit_job(job);
    }
    
        thread::sleep(Duration::from_secs(1));
        orchestrator.shutdown();
    }


    pub fn run_processor(processor: Processor) {
        match processor {
            Processor::Basic => simple_concurrent_processor(),
            Processor::Channel => run_channel_processor(),
        }
    }

    pub async fn run_async_processor() {
        println!("\n=== Async-based Orchestrator ===\n");
        let orchestrator = AsyncOrchestrator::new(4);
    
        for i in 0..8 {
        let job = AsyncJob {
            id: i,
            data: if i % 3 == 0 {
                format!("Process priority task {}", i)
            } else {
                format!("Process regular task {}", i)
            },  
            priority: if i % 3 == 0 { 0 } else { 1 },
        };
        orchestrator.submit_job(job).await;
    }
    
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_basic_processor() {
        run_processor(Processor::Basic);
    }

    #[test]
    fn test_channel_processor() {
        run_processor(Processor::Channel);
    }

    #[tokio::test]
    async fn test_async_processor() {
        run_async_processor().await;
    }
}