use crate::{Executable, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub name: String,
    pub executable: Box<dyn Executable>
}

impl Job {
    pub fn new(name: &str, executable: Box<dyn Executable>) -> Self { Self { name: name.to_string(), executable} }

    pub fn serialize(&self) -> Result<Vec<u8>> {
Ok(bincode::serialize(&self)?)
    }

    pub fn from_serialized(data: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize(data)?)
    }

}


impl std::fmt::Debug for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Job{{name: {:?}}}", self.name)?;
        Ok(())
    }
}