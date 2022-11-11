use crate::lib::run_example;
use std::error::Error;
use std::time::Duration;
use tokio_cron_scheduler_easy::{Job, JobScheduler};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod lib;
use tokio_cron_scheduler::JobToRun;

fn main() {
    let handle = std::thread::Builder::new()
        .name("schedule thread".to_string())
        .spawn(move || {
            // tokio::runtime::Builder::new_current_thread()    <- This hangs
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("build runtime failed")
                .block_on(start())
                .expect("TODO: panic message");
        })
        .expect("spawn thread failed");
    handle.join().expect("join failed");
}

async fn start() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");
    info!("Creating scheduler");
    let sched = JobScheduler::new().await?;
    info!("Run example");
    run_example(sched).await;
    Ok(())
}
