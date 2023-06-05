mod config;
mod load_balancing;

use crate::config::configure;
use crate::load_balancing::LoadBalancer;
use log::Level;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(Level::Debug)?;
    let global_configuration = configure("presentation.json")?;
    {
        let config = configure("presentation.json")?;
        log::info!("Using strategy: {:?}", config.strategy);
        log::info!("Load balancer listening on {:?}", config.address);
    }
    let listener = TcpListener::bind(global_configuration.address.clone()).await?;
    let load_balancer = LoadBalancer::new(global_configuration);

    while let Ok((mut ingress, addr)) = listener.accept().await {
        let mut egress =
            TcpStream::connect(load_balancer.get_receiver_ip(addr.to_string())).await?;

        tokio::io::copy_bidirectional(&mut ingress, &mut egress)
            .await
            .unwrap();
    }
    Ok(())
}
