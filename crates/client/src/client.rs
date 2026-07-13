use common::{recv, send};
use protocol::Request;
use std::io::Result;
use tokio::net::{
    TcpStream,
    tcp::{OwnedReadHalf, OwnedWriteHalf},
};

pub struct Client {
    read: OwnedReadHalf,
    write: OwnedWriteHalf,
}

impl Client {
    pub async fn connect(addr: &str) -> Result<Self> {
        let socket = TcpStream::connect(addr).await?;
        let (read, write) = socket.into_split();

        Ok(Self { read, write })
    }

    pub async fn login(&mut self, username: &str) -> Result<()> {
        let request = Request::set_name(username.to_string());

        send(&mut self.write, &request).await
    }

    pub async fn send(&mut self, request: &Request) -> Result<()> {
        send(&mut self.write, request).await
    }

    pub async fn recv(&mut self) -> Result<Request> {
        recv(&mut self.read).await
    }
}
