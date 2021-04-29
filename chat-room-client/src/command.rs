use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to determine command")]
    NotDetermined,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Clone, Debug)]
pub enum Command {
    MemoryStatistics,
}

impl Command {
    pub fn decode(string: &str) -> Result<Self> {
        match string {
            "memstats" => Ok(Command::MemoryStatistics),
            _ => Err(Error::NotDetermined),
        }
    }
}
