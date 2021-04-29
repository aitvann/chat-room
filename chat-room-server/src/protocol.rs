use crate::message::{ClientMessage, Command, Message};
use crate::room::{ShareReciever, ShareSender};
use futures::SinkExt;
use futures::StreamExt;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio_serde::{formats::SymmetricalJson, SymmetricallyFramed};
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

pub type InputStream<'a> = SymmetricallyFramed<
    FramedRead<ReadHalf<'a>, BytesCodec>,
    ClientMessage,
    SymmetricalJson<ClientMessage>,
>;
pub type OutputStream<'a> =
    SymmetricallyFramed<FramedWrite<WriteHalf<'a>, BytesCodec>, Message, SymmetricalJson<Message>>;

pub fn framed_stream(io_stream: &mut TcpStream) -> (InputStream<'_>, OutputStream<'_>) {
    let (read, write) = io_stream.split();

    let framed_read = FramedRead::new(read, BytesCodec::new());
    let read_codec = SymmetricalJson::<ClientMessage>::default();
    let input_stream: SymmetricallyFramed<_, ClientMessage, _> =
        SymmetricallyFramed::new(framed_read, read_codec);

    let framed_write = FramedWrite::new(write, BytesCodec::new());
    let write_codec = SymmetricalJson::<Message>::default();
    let output_stream: SymmetricallyFramed<_, Message, _> =
        SymmetricallyFramed::new(framed_write, write_codec);

    (input_stream, output_stream)
}