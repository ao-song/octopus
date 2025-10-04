use crate::broker::connection::Connection;
use log::{error, info};
use std::io;
use tokio::net::{TcpListener, TcpStream};

pub async fn start() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1883").await?;
    info!("MQTT broker listening on 127.0.0.1:1883");

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", addr);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                error!("Error handling connection from {}: {}", addr, e);
            }
        });
    }
}

async fn handle_connection(socket: TcpStream) -> io::Result<()> {
    let mut conn = Connection::new(socket);
    conn.read_packet().await?;
    Ok(())
}
