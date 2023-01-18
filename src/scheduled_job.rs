use crate::Executable;
use saffron::Cron;

pub struct ScheduledJob {
    pub schedule: Cron,
    pub create_instance: Box<dyn Fn() -> Box<dyn Executable> + Sync>,
}
