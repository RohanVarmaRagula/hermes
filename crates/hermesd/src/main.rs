use std::io::Result;
mod router;
mod server;
mod state;
mod user;

#[tokio::main]
async fn main() -> Result<()> {
    server::run().await?;

    Ok(())
}
