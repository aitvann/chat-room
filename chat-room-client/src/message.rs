use crate::command::Command;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
pub struct Message {
    pub addressee: String,
    pub text: String,
}

#[derive(Serialize, Clone, Debug)]
pub enum ClientMessage {
    Message(String),
    Command(Command),
}
