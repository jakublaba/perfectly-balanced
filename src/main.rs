mod config;
mod load_balancing;

use crate::config::configure;
use crate::load_balancing::LoadBalancer;
use log::Level;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(Level::Debug)?;
    let global_configuration = configure("src/config/test_resources/all_props.json")?;
    {
        let config = configure("src/config/test_resources/all_props.json")?;
        log::info!("Establishing a connection on {}", config.address.clone());
    }
    let listener = TcpListener::bind(global_configuration.address.clone()).await?;
    let load_balancer = LoadBalancer::new(global_configuration);

    while let Ok((mut ingress, addr)) = listener.accept().await {

        let mut egress = TcpStream::connect(load_balancer.get_receiver_ip(addr.to_string())).await?;

        match tokio::io::copy_bidirectional(&mut ingress, &mut egress).await {

            Ok((to_egress, to_ingress)) => {
                log::info!(
                        "Connection ended gracefully ({to_egress} bytes from client, {to_ingress} bytes from server)"
                    );
            }
            Err(err) => {
                log::info!("Error while proxying: {}", err);
            }
        }
    }
    Ok(())
}