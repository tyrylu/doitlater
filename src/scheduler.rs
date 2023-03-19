use crate::scheduled_job::ScheduledJob;
use crate::{Executable, Queue, Result};
use chrono::Utc;
use redis::{Client, Commands, Connection};
use std::collections::HashMap;

pub struct Scheduler {
    jobs: HashMap<String, ScheduledJob>,
    queue_name: String,
    redis_client: Client,
    redis_connection: Connection,
}

impl Scheduler {
    pub fn new(qname: &str, client: Client) -> Result<Self> {
        Ok(Scheduler {
            jobs: HashMap::new(),
            queue_name: qname.to_string(),
            redis_connection: client.get_connection()?,
            redis_client: client,
        })
    }

    pub fn register_job<F>(&mut self, name: &str, schedule: &str, create_instance: F) -> Result<()>
    where
        F: Fn() -> Box<dyn Executable> + 'static + Sync,
    {
        self.jobs.insert(
            name.to_string(),
            ScheduledJob {
                schedule: schedule.to_string(),
                create_instance: Box::new(create_instance),
            },
        );
        let score: Option<u64> = self
            .redis_connection
            .zscore(self.scheduled_jobs_key(), name)?;
        if score.is_none() {
            // We do not want to override the previous timestamp
            // The job_succeeded method does basically what we want, e. g. ensures that the job is scheduled in the future. And our name uniqueness contract ensures that it can be scheduled in the future only once.
            self.job_succeeded(name)?;
        }
        Ok(())
    }

    pub fn tick(&mut self) -> Result<()> {
        let current_timestamp = chrono::Utc::now().timestamp();
        let job_names = self.get_due_job_names(current_timestamp)?;
        let mut queue = Queue::from_client(&self.queue_name, self.redis_client.clone())?;
        for name in job_names {
            let job = self
                .jobs
                .get(&name)
                .expect("We did not know about a scheduled job");
            queue.enqueue(&name, (job.create_instance)())?;
        }
        self.redis_connection
            .zrembyscore(self.scheduled_jobs_key(), 0, current_timestamp)?;
        Ok(())
    }

    fn get_due_job_names(&mut self, timestamp: i64) -> Result<Vec<String>> {
        Ok(self
            .redis_connection
            .zrangebyscore(self.scheduled_jobs_key(), 0, timestamp)?)
    }

    fn scheduled_jobs_key(&self) -> String {
        format!("dil.{}.scheduled", self.queue_name)
    }

    pub fn job_succeeded(&mut self, name: &str) -> Result<()> {
        if let Some(job) = self.jobs.get(name) {
            let next_datetime = cron_parser::parse(&job.schedule, &Utc::now())?;
            self.redis_connection.zadd(
                self.scheduled_jobs_key(),
                name,
                next_datetime.timestamp(),
            )?;
        }
        Ok(())
    }
}
