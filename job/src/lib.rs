pub mod sealing;

use std::fmt::Display;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct JobId(pub Uuid);

impl JobId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<JobId> for Uuid {
    fn from(value: JobId) -> Self {
        value.0
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum JobType {
    PC1,
    PC2,
    C1,
    C2,
    PC,
}

impl Display for JobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobType::PC1 => f.write_str("PC1"),
            JobType::PC2 => f.write_str("PC2"),
            JobType::C1 => f.write_str("C1"),
            JobType::C2 => f.write_str("C2"),
            JobType::PC => f.write_str("PC"),
        }
    }
}

impl From<JobType> for u8 {
    fn from(value: JobType) -> Self {
        match value {
            JobType::PC1 => 1,
            JobType::PC2 => 2,
            JobType::C1 => 3,
            JobType::C2 => 4,
            JobType::PC => 5,
        }
    }
}

pub trait Job: Serialize + DeserializeOwned + Send + Sync {
    type Input: Send + Sync + Serialize + DeserializeOwned;
    type Output: Send + Sync + Serialize + DeserializeOwned + AsRef<[u8]> + From<Vec<u8>>;
    type Metadata: Send + Sync + Serialize + DeserializeOwned;

    fn job_type() -> JobType;
    fn into_input(self) -> Self::Input;
    fn metadata(&self) -> Self::Metadata;
    fn domain_id(&self) -> String;
}
