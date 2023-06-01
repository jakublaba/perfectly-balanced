mod config;
mod load_balancing;

use log::Level;
use tokio::net::TcpListener;
use crate::config::configure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(Level::Debug)?;
    let global_configuration = configure("src/config/test_resources/all_props.json")?;
    let listener = TcpListener::bind(global_configuration.address.clone()).await?;
    log::debug!("Establishing a connection on {}", global_configuration.address.clone());
    loop {
        let (socket, _) = listener.accept().await?;
        process_incoming_request(socket).await;
    }

    Ok(())
}

async fn process_incoming_request<T>(socket: T) {
    log::debug!("Connection has been established");
}
