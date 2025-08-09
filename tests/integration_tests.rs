#[cfg(test)]
mod tests {
    use rust_concurrent_image::basic_processor::basic_concurrent_processor::simple_concurrent_processor;
    use rust_concurrent_image::channel_processor::channel_concurrent_processor::{Job, Orchestrator};
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

    #[test]
    fn test_basic_processor() {
        run_processor(Processor::Basic);
    }

    #[test]
    fn test_channel_processor() {
        run_processor(Processor::Channel);
    }
}