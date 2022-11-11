use crate::job_scheduler::JobsSchedulerLocked;
use crate::{Job, JobScheduler};
use uuid::Uuid;

/// 简化创建crontab功能而已。
///
/// #example
/// ```rust
/// let cron = CronJob::new().await;
/// let ms = Arc::clone(&market_server);
/// // 类crontab格式： 秒 分 时 日 月 周 [年]
/// cron.add("0 40 8,12,20 * * 1-5", move |_u, _l| {
///     println!("调度器触发...");
/// })
/// .await;
/// cron.start().await;
pub struct CronJob {
    scheduler: JobsSchedulerLocked,
}

impl CronJob {
    pub async fn new() -> Self {
        let sched = JobScheduler::new().await.unwrap();
        Self { scheduler: sched }
    }

    /// add cronjob
    pub async fn add<F>(&self, expr: &str, job: F)
    where
        F: FnMut(Uuid, JobsSchedulerLocked) + Send + Sync + 'static,
    {
        self.scheduler
            .add(Job::new(expr, job).unwrap())
            .await
            .unwrap();
    }

    /// start cronjob
    pub async fn start(&self) {
        self.scheduler.start().await.unwrap();
    }
}
