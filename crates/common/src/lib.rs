use protocol::{read_frame, write_frame, Request};
use std::io::{Error, ErrorKind, Result};
use tokio::{
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

pub async fn send(
    socket: &mut OwnedWriteHalf,
    request: &Request
) -> Result<()> {
    let msg = request
        .to_string()
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    
    write_frame(socket, &msg).await    
}

pub async fn relay_message(
    socket: &mut OwnedWriteHalf, 
    msg: &str
) -> Result<()> {
    write_frame(socket, msg).await
}

pub async fn recv(
    socket: &mut OwnedReadHalf
) -> Result<Request> {
    let text = read_frame(socket).await?;
    Request::from(text)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

pub async fn recv_relay_message(
    socket: &mut OwnedReadHalf
) -> Result<String> {
    read_frame(socket).await
}
