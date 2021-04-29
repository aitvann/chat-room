use crate::{message::Message, user::User};
use std::io;
use std::net::SocketAddr;
use tokio::{net::TcpListener, sync::broadcast};

pub type ShareReciever = broadcast::Receiver<Message>;
pub type ShareSender = broadcast::Sender<Message>;

#[derive(Debug)]
pub struct Room {
    listener: TcpListener,

    share_sender: ShareSender,
    share_reciever: ShareReciever,
}

impl Room {
    pub async fn start(addr: SocketAddr) -> io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        let (share_sender, share_reciever) = broadcast::channel(256);
        let room = Self {
            listener,
            share_sender,
            share_reciever,
        };

        Ok(room)
    }

    pub async fn run(&mut self) {
        loop {
            let connection = match self.listener.accept().await {
                Ok((connection, _)) => connection,
                Err(e) => {
                    log::error!("failed to accept connection: {}", e);
                    continue;
                }
            };

            log::trace!("connection accepted");

            let user = User::new(
                connection,
                self.share_sender.clone(),
                self.share_sender.subscribe(),
            ).await;

            log::trace!("user created");

            tokio::spawn(user.run());
        }
    }
}
