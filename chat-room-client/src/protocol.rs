use crate::message::{ClientMessage, Message};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio_serde::{formats::SymmetricalJson, SymmetricallyFramed};
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

pub type InputStream<'a> =
    SymmetricallyFramed<FramedRead<ReadHalf<'a>, BytesCodec>, Message, SymmetricalJson<Message>>;
pub type OutputStream<'a> = SymmetricallyFramed<
    FramedWrite<WriteHalf<'a>, BytesCodec>,
    ClientMessage,
    SymmetricalJson<ClientMessage>,
>;

pub fn framed_stream(io_stream: &mut TcpStream) -> (InputStream<'_>, OutputStream<'_>) {
    let (read, write) = io_stream.split();

    let framed_read = FramedRead::new(read, BytesCodec::new());
    let read_codec = SymmetricalJson::default();
    let input_stream = SymmetricallyFramed::new(framed_read, read_codec);

    let framed_write = FramedWrite::new(write, BytesCodec::new());
    let write_codec = SymmetricalJson::default();
    let output_stream = SymmetricallyFramed::new(framed_write, write_codec);

    (input_stream, output_stream)
}
