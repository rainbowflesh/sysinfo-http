mod app;
mod server;
use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()> {
    // tokio::spawn(msg_server::start());
    server::start_server().await;
    Ok(())
}
