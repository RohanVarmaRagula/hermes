use std::io::Result;
mod client;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    // client::run().await?;
    ui::run::run()?;
    Ok(())
}
