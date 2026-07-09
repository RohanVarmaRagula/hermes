use crate::{state::ServerState, user::User};
use common::relay_message;
use protocol::{Command, Request};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn route_request(
    user: &User,
    server_state: &Arc<Mutex<ServerState>>,
    request: Request,
) -> Result<(), String> {
    match request.command {
        Command::SetName => Err("User is already registered".to_string()),
        Command::SendToPeer => send_to_peer(user, server_state, request).await,
        Command::SendToRoom => send_to_room(user, server_state, request).await,
        Command::Unknown => Err(String::from("Unknown command")),
    }
}

async fn send_to_peer(
    user: &User,
    server_state: &Arc<Mutex<ServerState>>,
    request: Request,
) -> Result<(), String> {
    let writer = {
        let state = server_state.lock().await;
        state.get_writer(&request.target)?
    };

    let msg = format!("{}: {}", user.name, request.message);

    let mut writer = writer.lock().await;
    relay_message(&mut writer, &msg)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn send_to_room(
    _user: &User,
    _server_state: &Arc<Mutex<ServerState>>,
    _request: Request,
) -> Result<(), String> {
    unimplemented!()
}
