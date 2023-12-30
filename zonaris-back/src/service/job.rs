use anyhow::Result;
use async_trait::async_trait;
use log::error;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[async_trait]
pub trait Job
where
    Self: Send + Sync + 'static,
{
    async fn job_func(job_state: Arc<RwLock<Self>>) -> Result<()>;

    fn create_job(self, time_step: Duration) -> Result<tokio_cron_scheduler::Job>
    where
        Self: Sized,
    {
        let job_state = Arc::new(RwLock::new(self));

        let job = tokio_cron_scheduler::Job::new_repeated_async(
            time_step,
            move |_uuid, _job_scheduler| {
                let job_state = job_state.clone();

                return Box::pin(async move {
                    if let Err(err) = Self::job_func(job_state).await {
                        error!("{}\n{}", err, err.backtrace());
                    }
                });
            },
        )?;

        return Ok(job);
    }
}
