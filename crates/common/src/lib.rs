use protocol::{Request, read_frame, write_frame};
use std::{
    io::{Error, ErrorKind, Result},
    str::FromStr,
};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub async fn send(socket: &mut OwnedWriteHalf, request: &Request) -> Result<()> {
    let text = request.to_string();
    write_frame(socket, &text).await
}

pub async fn recv(socket: &mut OwnedReadHalf) -> Result<Request> {
    let text = read_frame(socket).await?;

    Request::from_str(&text).map_err(|e| Error::new(ErrorKind::InvalidData, e))
}
