use common::relay_message;
use protocol::{Command, Request};
use crate::{state::ServerState, user::User};

pub async fn route_request(
    user: &User, 
    server_state: &ServerState, 
    request: Request
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
    server_state: &ServerState, 
    request: Request
) -> Result<(), String> {
    let writer = server_state.get_writer(&request.target)?;
    let mut writer = writer.lock().await;
    let msg = format!("{}: {}", user.name, request.message);
    relay_message(&mut writer, &msg)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

async fn send_to_room(
    _user: &User,
    _server_state: &ServerState, 
    _request: Request
) -> Result<(), String> {
    unimplemented!()
}