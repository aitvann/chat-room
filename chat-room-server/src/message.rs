use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
pub struct Message {
    pub addressee: String,
    pub text: String,
}

#[derive(Deserialize, Clone, Debug)]
pub enum ClientMessage {
    Message(String),
    Command(Command),
}

#[derive(Deserialize, Clone, Debug)]
pub enum Command {
    MemoryStatistics,
}
