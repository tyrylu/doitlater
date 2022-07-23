use doitlater::{ExecutableExt, Queue, Worker, typetag};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct HelloTask {
    recipient: String
}

#[typetag::serde]
impl doitlater::Executable for HelloTask {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Oh, hello, {}.", self.recipient);
        Ok(())
    }
}

impl HelloTask {
    pub fn new(recipient: &str) -> Self {
        Self {recipient: recipient.to_string()}
    }
}

fn main() -> Result<(), doitlater::error::Error> {
    let mut queue = Queue::new("tasks", "redis://localhost")?;
    HelloTask::new("John").enqueue_into(&mut queue, "SayHimHello")?;
    let mut worker = Worker::new("tasks", "redis://localhost")?;
    let mut scheduler = worker.create_scheduler()?;
    scheduler.register_job("SayHelloOften", "* * * * *", || Box::new(HelloTask::new("our repeating world")));
    worker.use_scheduler(scheduler);    
    worker.run()?;
    Ok(())
}
