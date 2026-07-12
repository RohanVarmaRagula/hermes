use crate::{router, state::ServerState, user::User};
use common::recv;
use protocol::{Command, Request};
use std::env;
use std::{io::Result, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

async fn process_socket(socket: TcpStream, state: Arc<Mutex<ServerState>>) -> Result<()> {
    let (mut read_h, write_h) = socket.into_split();
    let request = recv(&mut read_h).await?;
    let user = User::new(request.target.clone(), Arc::new(Mutex::new(write_h)));
    {
        let mut server_state = state.lock().await;
        server_state
            .enroll_user(user.clone())
            .map_err(std::io::Error::other)?;
    }
    loop {
        match recv(&mut read_h).await {
            Ok(request) => {
                if let Err(e) = router::route_request(&user, &state, request).await {
                    router::route_request(
                        &user,
                        &state,
                        Request::new(Command::SendToPeer, user.name.to_string(), e.to_string()),
                    )
                    .await
                    .unwrap();
                }
            }
            Err(_) => {
                {
                    let mut server_state = state.lock().await;
                    server_state.remove_user(&user.name);
                }
                break;
            }
        };
    }

    Ok(())
}

pub async fn run() -> Result<()> {
    let state = Arc::new(Mutex::new(ServerState::new()));

    let args: Vec<String> = env::args().collect();
    let addr = if args.len() < 2 {
        "0.0.0.0:8080"
    } else {
        &args[1]
    };
    let listener = TcpListener::bind(addr).await?;
    println!("Server is listening on {:?}", listener.local_addr()?);

    loop {
        let (socket, _) = TcpListener::accept(&listener).await?;
        let state_clone = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(e) = process_socket(socket, state_clone).await {
                eprint!("{e}");
            }
        });
    }
}
