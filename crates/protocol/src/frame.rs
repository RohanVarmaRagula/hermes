use std::io::Result;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::tcp::{OwnedReadHalf, OwnedWriteHalf}};

pub async fn write_frame(
    socket: &mut OwnedWriteHalf,
    text: &str,
) -> Result<()> {
    socket.write_u32(text.len() as u32).await?;
    socket.write_all(text.as_bytes()).await?;

    Ok(())
}

pub async fn read_frame(
    socket: &mut OwnedReadHalf,
) -> Result<String> {
    let len = socket.read_u32().await?;

    let mut buf = vec![0u8; len as usize];
    socket.read_exact(&mut buf).await?;

    Ok(String::from_utf8_lossy(&buf).to_string())
}