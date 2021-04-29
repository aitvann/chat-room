use crate::message::{ClientMessage, Command, Message};
use crate::room::{ShareReciever, ShareSender};
use futures::SinkExt;
use futures::StreamExt;
use tokio::net::TcpStream;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Debug)]
pub struct User {
    name: String,

    io_stream: TcpStream,

    share_sender: ShareSender,
    share_reciever: ShareReciever,
}

impl User {
    pub async fn new(
        mut io_stream: TcpStream,
        share_sender: ShareSender,
        share_reciever: ShareReciever,
    ) -> Self {
        let (mut istream, _) = crate::protocol::framed_stream(&mut io_stream);

        // Waits for the user to send their name
        let name = loop {
            if let Some(Ok(ClientMessage::Message(msg))) = istream.next().await {
                break msg;
            }
        };

        log::trace!("username recieved");

        Self {
            name,
            share_sender,
            share_reciever,
            io_stream,
        }
    }

    pub async fn run(mut self) {
        let (mut istream, mut ostream) = crate::protocol::framed_stream(&mut self.io_stream);

        loop {
            tokio::select! {
                Some(Ok(client_message)) = istream.next() => {
                    match client_message {
                        ClientMessage::Message(text) => {
                            log::trace!("message recieved");
                            let addressee = self.name.clone();
                            let msg = Message { addressee, text };
                            let _ = self.share_sender.send(msg); // ignore fail
                        },
                        ClientMessage::Command(cmd) => {
                            log::trace!("command recieved");
                            let text = Self::handle_command(cmd);
                            let addressee = "Server".to_string();
                            let msg = Message { addressee, text };
                            let _ = dbg!(ostream.send(msg).await); // ignore fail
                        },
                    }
                },
                Ok(message) = self.share_reciever.recv() => {
                    if message.addressee == self.name {
                        continue;
                    }

                    let _ = ostream.send(message).await; // ignore fail
                }
            }
        }
    }

    pub fn handle_command(cmd: Command) -> String {
        match cmd {
            Command::MemoryStatistics => format!("{}\n", crate::utils::memory_usage()),
        }
    }
}

