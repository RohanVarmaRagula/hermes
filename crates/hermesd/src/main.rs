use std::io::Result;
mod server;
mod router;
mod state;
mod user;

#[tokio::main]
async fn main() -> Result<()> {
    server::run().await?;

    Ok(())
}
