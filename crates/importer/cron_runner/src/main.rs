use importer::settings::config::Settings;
use importer::jobs::import_csv::{ImportCsvJob, import_merchants_csv_handler};
use tokio_cron_scheduler::{Job, JobScheduler};
use tokio::sync::oneshot;
use std::time::Duration;
use importer::events::kafka::publisher::KafkaPublisherBuilder;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    if let Err(e) = start_scheduler_once().await {
        eprintln!("Job error: {:?}", e);
    }

    println!("Shutting down gracefully...");
}

async fn start_scheduler_once() -> Result<(), Box<dyn std::error::Error>> {
    let mut scheduler = JobScheduler::new().await?;

    let (tx, rx) = oneshot::channel();

    let mut tx = Some(tx);

    // job that runs only once, after the specified time (Duration)
    // https://docs.rs/tokio-cron-scheduler/latest/tokio_cron_scheduler/job/struct.JobLocked.html#method.new_one_shot_async
    // i'm not gonna deal in this exercise with retries...etc, that's other interesting topic.

    let job = Job::new_one_shot_async(Duration::ZERO, move |job_id, _lock| {
        let settings = Settings::from_env();
        let tx_opt = tx.take();

        Box::pin(async move {
            println!("🛠 Running the job with ID: {}", job_id);

            let result = run_import_csv_job(settings).await;

            if let Err(err) = result {
                eprintln!("❌ Job {} failed: {:#}", job_id, err);
            }

            if let Some(sender) = tx_opt {
                let _ = sender.send(());
            }
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    rx.await.ok();

    scheduler.shutdown().await?;

    Ok(())
}

async fn run_import_csv_job(settings: Settings) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file_path = &settings.csv_path;
    let job = ImportCsvJob { file_path: file_path.to_string() };
    let publisher = KafkaPublisherBuilder::new(&settings.kafka_brokers).build()?;

    import_merchants_csv_handler(job, &publisher, settings).await?;

    Ok(())
}