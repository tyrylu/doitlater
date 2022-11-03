use std::{sync::atomic::{AtomicBool, Ordering}, time::{Duration, Instant}};
use crate::{Queue, Result, error::Error, job::Job, scheduler::Scheduler};
use std::env;
use log::{error, info};

pub struct Worker {
    queue: Queue,
    should_exit: AtomicBool,
    scheduler: Option<Scheduler>,
}

impl Worker {
    pub fn new(queue_name: &str, redis_connection_string: &str) -> Result<Self> {
        Ok(Self{queue: Queue::new(queue_name, redis_connection_string)?, should_exit: AtomicBool::new(false), scheduler: None})
    }

    pub fn new_from_env() -> Result<Self> {
        Self::new(&env::var("QUEUE_NAME").expect("QUEUE_NAME should be set"), &env::var("REDIS_URL").expect("REDIS_URL should be set"))
    }

    fn maybe_process_job(&mut self, timeout: usize) -> Result<Option<Job>> {
        let maybe_job = self.queue.dequeue_executable_job(timeout)?;
        match maybe_job {
            Some(job) => {
                    if let Err(e) = job.executable.execute() {
            Err(Error::JobExecutionError{job_name: job.name, error: e.to_string()})
        }
        else {
            Ok(Some(job))
        }
    },
    None => Ok(None)
}
    }

    pub fn run(&mut self) -> Result<()> {
        let mut now = Instant::now();
        let mut first = true;
        while !self.should_exit.load(Ordering::SeqCst) {
            if let Some(scheduler) = &mut self.scheduler {
                if first || now.elapsed() > Duration::from_secs(60) {
                    scheduler.tick()?;
                    first = false;
                    now = Instant::now();
                }
            }
            match self.maybe_process_job(10) {
                Ok(None) => continue,
                Ok(Some(job)) => {
                    info!("Successfully executed {}.", job.name);
                    self.queue.delete_job_data(&job.name)?;
                    if let Some(scheduler) = &mut self.scheduler {
                        scheduler.job_succeeded(&job.name)?;
                    }
                },
                Err(Error::JobExecutionError{job_name, error}) => {
                    error!("Failed to execute job {} with error: {}.", job_name, error);
                    self.queue.delete_job_data(&job_name)?;
                },
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }

    pub fn request_shutdown(&self) {
        self.should_exit.store(true, Ordering::SeqCst);
        }

    pub fn create_scheduler(&self) -> Result<Scheduler> {
                        Scheduler::new(self.queue.name(), self.queue.get_client_clone())
    }

    pub fn use_scheduler(&mut self, scheduler: Scheduler) {
        self.scheduler = Some(scheduler);
    }
}