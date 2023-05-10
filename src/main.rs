use log::Level;

async fn hello() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    log::info!("Hello, world!");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(Level::Info)?;

    tokio::join!(
        hello(),
        hello(),
        hello(),
        hello(),
        hello(),
    );

    Ok(())
}
