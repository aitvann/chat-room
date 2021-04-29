mod command;
mod message;
mod protocol;

use crate::command::Command;
use crate::message::{ClientMessage, Message};
use futures::SinkExt;
use futures::StreamExt;
use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr};
use structopt::StructOpt;
use tokio::io::{self as aio, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[derive(Debug, StructOpt)]
#[structopt(name = "Chat client", about = "A client for chating")]
struct Opt {
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: IpAddr,

    #[structopt(short, long, default_value = "4901")]
    port: u16,
}

async fn print_message(message: Message) -> anyhow::Result<()> {
    let formated_message = format!("{}: {}", message.addressee, message.text);
    let mut stdout = tokio::io::stdout();

    stdout
        .write_all(formated_message.as_bytes())
        .await?;

    stdout.flush().await?;

    Ok(())
}

async fn print_error(error: anyhow::Error) -> anyhow::Result<()> {
    let addressee = "System".to_string();
    let text = error.to_string();
    let message = Message { addressee, text };
    print_message(message).await
}

async fn get_input() -> anyhow::Result<ClientMessage> {
    let mut input = String::new();
    let mut reader = BufReader::new(aio::stdin());
    reader.read_line(&mut input).await?;

    if input.starts_with('/') {
        let cmd = Command::decode(&input[1..].trim())?;
        Ok(ClientMessage::Command(cmd))
    } else {
        log::trace!("message entered");
        Ok(ClientMessage::Message(input))
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    log::trace!("Starting up");

    let opt = Opt::from_args();
    let addr = SocketAddr::new(opt.host, opt.port);
    let mut io_stream = TcpStream::connect(addr)
        .await
        .expect("failed to connect to the server");

    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("failed to read name");

    let (mut istream, mut ostream) = protocol::framed_stream(&mut io_stream);

    let msg = ClientMessage::Message(name.trim().to_string());
    ostream.send(msg).await.expect("failed to send username");

    loop {
        tokio::select! {
            Ok(input) = tokio::spawn(get_input()) => {
                match input {
                    Ok(client_message) => {
                        let _ = ostream.send(client_message).await; // ignore fail
                        log::trace!("message send");
                    },
                    Err(e) => {
                        let _ = print_error(e).await; // ignore fail
                    }
                }
            },
            Some(Ok(message)) = istream.next() => {
                let _ = print_message(message).await; // ignore fail
            }
        }
    }
}
