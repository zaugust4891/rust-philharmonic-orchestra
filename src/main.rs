use rust_concurrent_image::basic_processor::basic_concurrent_processor::simple_concurrent_processor;
use rust_concurrent_image::channel_processor::channel_concurrent_processor::{Job, Orchestrator};

pub enum Processor {
    Basic,
    Channel,
}

pub fn run_basic_processor() {
    simple_concurrent_processor();
}
pub fn run_channel_processor() {
    let orchestrator = Orchestrator::new(10);
    for i in 0..100 {
        let job = Job {
            id: i,
            data: format!("Processing job {}", i),
        };
        orchestrator.submit_job(job);
    }
    orchestrator.shutdown();
}


pub fn run_processor(processor: Processor) {
    match processor {
        Processor::Basic => simple_concurrent_processor(),
        Processor::Channel => run_channel_processor(),
    }
}
fn main() {
    run_processor(Processor::Channel);

}
