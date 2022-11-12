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

    /// add cronjob. expr格式说明：
    ///
    /// 秒  分  时 日   月   周  [年]
    ///
    /// 0 1/2  1  2  1-12 1-7  2022
    /// 特别注意，周用数字表示是，周日为1，周一到周六为2-7。可以用英文表示：
    /// "sun" | "sunday" => 1,
    /// "mon" | "monday" => 2,
    /// "tue" | "tues" | "tuesday" => 3,
    /// "wed" | "wednesday" => 4,
    /// "thu" | "thurs" | "thursday" => 5,
    /// "fri" | "friday" => 6,
    /// "sat" | "saturday" => 7,
    ///
    /// - expr: 任务的时间表达式
    /// - job：任务执行器
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
