pub mod error;
pub mod executable;
mod job;
pub mod queue;
mod scheduled_job;
mod scheduler;
pub mod worker;

pub type Result<T> = std::result::Result<T, error::Error>;
pub use error::Error;
pub use executable::{Executable, ExecutableExt};
pub use queue::Queue;
pub use scheduler::Scheduler;
pub use typetag;
pub use worker::Worker;
