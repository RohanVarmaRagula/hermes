use common::{recv_relay_message, send};
use protocol::Request;
use std::env;
use std::io::{Result, Write};
use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    net::TcpStream,
};

use crate::client;

async fn process_socket(socket: TcpStream) -> Result<()> {
    let (mut read_h, mut write_h) = socket.into_split();

    print!("Enter your name: ");
    std::io::stdout().flush().unwrap();

    let mut reader = BufReader::new(io::stdin());

    let mut name = String::new();
    reader.read_line(&mut name).await?;
    let name = name.trim();

    let name_request = Request::from(format!("/name {name}")).map_err(std::io::Error::other)?;
    send(&mut write_h, &name_request).await?;

    let send_handle = tokio::spawn(async move {
        // send msgs
        let mut reader = reader;

        loop {
            print!("@client> ");
            std::io::stdout().flush().unwrap();

            let mut msg = String::new();

            if let Err(e) = reader.read_line(&mut msg).await {
                eprintln!("stdin error: {e}");
                break;
            }

            let msg = msg.trim();
            if msg.is_empty() {
                continue;
            }

            let request = match Request::from(msg.to_string()) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{e}");
                    continue;
                }
            };

            if let Err(e) = send(&mut write_h, &request).await {
                eprintln!("Send error: {e}");
                break;
            }
        }
    });

    let recv_handle = tokio::spawn(async move {
        // recv msgs
        loop {
            match recv_relay_message(&mut read_h).await {
                Ok(msg) => {
                    println!("{}", msg);
                }
                Err(e) => {
                    eprintln!("Receive error: {e}");
                    break;
                }
            };
        }
    });

    send_handle.await.unwrap();
    recv_handle.await.unwrap();

    Ok(())
}

pub async fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let addr = if args.len() < 2 {
        "127.0.0.1:8080"
    } else {
        &args[1]
    };
    let socket = TcpStream::connect(addr).await?;
    println!("Connected to server");

    process_socket(socket).await
}
