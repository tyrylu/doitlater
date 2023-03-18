use crate::Executable;

pub struct ScheduledJob {
    pub schedule: String,
    pub create_instance: Box<dyn Fn() -> Box<dyn Executable> + Sync>,
}
