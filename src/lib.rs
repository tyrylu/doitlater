pub mod error;
pub mod executable;
pub mod queue;
mod job;
pub mod worker;
mod scheduled_job;
mod scheduler;

pub type Result<T> = std::result::Result<T, error::Error>;
pub use  executable::{Executable, ExecutableExt};
pub use worker::Worker;
pub use queue::Queue;
pub use scheduler::Scheduler;
pub use error::Error;
pub use typetag;