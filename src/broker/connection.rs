use crate::broker::packets::Packet;
use bytes::BytesMut;
use std::io;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn read_packet(&mut self) -> io::Result<Option<Packet>> {
        loop {
            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                return if self.buffer.is_empty() { Ok(None) } else { Err(io::Error::new(io::ErrorKind::ConnectionReset, "connection reset by peer")) };
            }

            // TODO: Implement packet parsing logic here.
        }
    }
}
