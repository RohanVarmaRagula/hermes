use protocol::Request;
use std::io::{Error, ErrorKind, Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

pub async fn send(socket: &mut OwnedWriteHalf, request: &Request) -> Result<()> {
    let msg = request
        .to_string()
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

    socket.write_all(msg.as_bytes()).await
}

pub async fn relay_message(socket: &mut OwnedWriteHalf, msg: &str) -> Result<()> {
    socket.write_all(msg.as_bytes()).await
}

pub async fn recv(socket: &mut OwnedReadHalf) -> Result<Request> {
    let mut buff = [0u8; 1024];
    let n = socket.read(&mut buff).await?;
    if n == 0 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Connection Closed"));
    }

    let text = String::from_utf8_lossy(&buff[..n]).trim().to_string();

    Request::from(text).map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

pub async fn recv_relay_message(socket: &mut OwnedReadHalf) -> Result<String> {
    let mut buf = [0u8; 1024];

    let n = socket.read(&mut buf).await?;

    if n == 0 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Connection closed"));
    }

    Ok(String::from_utf8_lossy(&buf[..n]).trim().to_string())
}
