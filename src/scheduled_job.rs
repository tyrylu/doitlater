use saffron::Cron;
use crate::Executable;

pub struct ScheduledJob {
    pub schedule: Cron,
    pub create_instance: Box<dyn Fn() -> Box<dyn Executable> + Sync>
}