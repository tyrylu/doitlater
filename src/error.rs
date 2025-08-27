/// Represents the various errors which can happen during the operation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::error::EncodeError),
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] bincode::error::DecodeError),
    #[error("Job execution error: {error}")]
    JobExecutionError { job_name: String, error: String },
    #[error("Cron parsing error: {0}")]
    CronParsingError(#[from] cron_parser::ParseError),
}
