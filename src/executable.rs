use crate::Queue;
use std::error::Error;

/// Represents an executable background job.
#[typetag::serde]
pub trait Executable {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}

pub trait ExecutableExt {
    fn enqueue_into(self, queue: &mut Queue, name: &str) -> crate::Result<()>;
}

impl<T: Executable + 'static> ExecutableExt for T {
    fn enqueue_into(self, queue: &mut Queue, name: &str) -> crate::Result<()> {
        queue.enqueue(name, Box::new(self))
    }
}
