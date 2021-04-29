use std::net::{IpAddr, SocketAddr};

use structopt::StructOpt;

mod room;
use room::Room;

mod message;
mod user;
mod protocol;
mod utils;

#[derive(Debug, StructOpt)]
#[structopt(name = "Chat room", about = "A room for chating")]
struct Opt {
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: IpAddr,

    #[structopt(short, long, default_value = "4901")]
    port: u16,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    log::trace!("Starting up");

    let opt = Opt::from_args();
    let addr = SocketAddr::new(opt.host, opt.port);
    let mut room = Room::start(addr).await.expect("failed to start a room");

    log::trace!("Running");
    room.run().await;

    log::trace!("Shutting down")
}
