/// Represents the various errors which can happen during the operation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Serialization error: {0}")]
    BincodeError(#[from] Box<bincode::ErrorKind>),
    #[error("Job execution error: {error}")]
    JobExecutionError{job_name: String, error: String},
    #[error("Cron parsing error: {0}")]
    CronParsingError(saffron::parse::CronParseError)
}