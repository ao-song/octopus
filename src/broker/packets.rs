use bytes::Bytes;

#[derive(Debug)]
pub enum Packet {
    Connect(Connect),
}

#[derive(Debug)]
pub struct Connect {
    pub payload: Bytes,
}
