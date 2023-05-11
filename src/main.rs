mod config;

use log::Level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(Level::Debug)?;

    Ok(())
}
