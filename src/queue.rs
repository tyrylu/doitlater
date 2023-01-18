use crate::{job::Job, Executable, Result};
use redis::{Client, Commands, Connection, LposOptions};
use std::env;

pub struct Queue {
    qname: String,
    client: Client,
    redis_conn: Connection,
}

impl Queue {
    pub fn new(name: &str, redis_connection_string: &str) -> Result<Self> {
        let client = Client::open(redis_connection_string)?;
        let conn = client.get_connection()?;
        Ok(Queue {
            qname: name.to_string(),
            redis_conn: conn,
            client,
        })
    }

    pub fn from_client(name: &str, client: Client) -> Result<Self> {
        let conn = client.get_connection()?;
        Ok(Queue {
            qname: name.to_string(),
            redis_conn: conn,
            client,
        })
    }

    pub fn new_from_env() -> Result<Self> {
        Self::new(
            &env::var("QUEUE_NAME").expect("QUEUE_NAME should be set"),
            &env::var("REDIS_URL").expect("REDIS_URL should be set"),
        )
    }

    fn jobs_storage_key(&self) -> String {
        format!("dil.{}.jobs", self.qname)
    }

    fn immediately_executable_key(&self) -> String {
        format!("dil.{}.executable", self.qname)
    }

    pub fn enqueue(&mut self, name: &str, executable: Box<dyn Executable>) -> Result<()> {
        let pos: Option<isize> = self.redis_conn.lpos(
            self.immediately_executable_key(),
            name,
            LposOptions::default(),
        )?;
        if pos.is_none() {
            let job = Job::new(name, executable);
            redis::pipe()
                .atomic()
                .rpush(self.immediately_executable_key(), name)
                .ignore()
                .hset(self.jobs_storage_key(), name, job.serialize()?)
                .ignore()
                .query(&mut self.redis_conn)?;
        }
        Ok(())
    }

    pub fn dequeue_executable_job(&mut self, timeout: usize) -> Result<Option<Job>> {
        let result: Option<(String, String)> = self
            .redis_conn
            .blpop(self.immediately_executable_key(), timeout)?;
        if let Some((_key, name)) = result {
            let data: Vec<u8> = self.redis_conn.hget(self.jobs_storage_key(), name)?;
            Ok(Some(Job::from_serialized(&data)?))
        } else {
            Ok(None)
        }
    }

    pub fn delete_job_data(&mut self, name: &str) -> Result<()> {
        self.redis_conn.hdel(self.jobs_storage_key(), name)?;
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.qname
    }

    pub fn get_client_clone(&self) -> Client {
        self.client.clone()
    }
}
