use octopus::broker::server;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    server::start().await
}
