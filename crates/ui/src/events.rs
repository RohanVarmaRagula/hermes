// handles events

use crate::{
    app::{App, Focus},
    contacts::{Contact, ContactType},
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use protocol::{Command, Request};
use ratatui::layout::Position;
use std::io;

pub enum Action {
    Continue,
    Quit,
}

fn handle_event_peer_contact_area(app: &mut App, event: Event) -> io::Result<Action> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Down => app.peer_contacts.select_next(),
            KeyCode::Up => app.peer_contacts.select_prev(),
            _ => {}
        },
        Event::Mouse(mouse) => match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let index = mouse.row - app.peer_contact_list_area.y - 1;
                app.peer_contacts.select(Some(index as usize));
                app.room_contacts.select(None);
            }
            _ => {}
        },
        _ => {}
    }
    Ok(Action::Continue)
}

fn handle_event_room_contact_area(app: &mut App, event: Event) -> io::Result<Action> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Down => app.room_contacts.select_next(),
            KeyCode::Up => app.room_contacts.select_prev(),
            _ => {}
        },
        Event::Mouse(mouse) => match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let index = mouse.row - app.room_contact_list_area.y - 1;
                app.room_contacts.select(Some(index as usize));
                app.peer_contacts.select(None);
            }
            _ => {}
        },
        _ => {}
    }
    Ok(Action::Continue)
}

fn handle_event_chat_area(app: &mut App, event: Event) -> io::Result<Action> {
    unimplemented!();
}

async fn handle_event_input_box_area(app: &mut App, event: Event) -> io::Result<Action> {
    if let Event::Key(key) = event {
        if key.code == KeyCode::Enter && key.kind == KeyEventKind::Press {
            let target = {
                if let Some(peer_contact) = app.peer_contacts.selected() {
                    peer_contact
                } else if let Some(room_contact) = app.room_contacts.selected() {
                    room_contact
                } else {
                    &Contact::new("None", ContactType::Peer)
                }
            };
            let lines = app.textarea.lines().join("\n");

            let request = {
                let words: Vec<String> = lines.split_whitespace().map(str::to_string).collect();
                if words[0] == "/add_peer" {
                    Request::add_peer(words[1..].join("-"))
                } else if words[0] == "/add_room" {
                    Request::add_room(words[1..].join("-"))
                } else if target.contact_type == ContactType::Peer {
                    Request::send_to_peer(&target.name, lines)
                } else if target.contact_type == ContactType::Room {
                    Request::send_to_room(&target.name, lines)
                } else {
                    unimplemented!()
                }
            };

            match request.command {
                Command::AddPeer => {
                    let contact = Contact::new(request.target, ContactType::Peer);
                    if !app.peer_contacts.contacts.contains(&contact) {
                        app.peer_contacts.contacts.push(contact);
                    }
                }
                Command::AddRoom => {
                    let contact = Contact::new(request.target, ContactType::Room);
                    if !app.room_contacts.contacts.contains(&contact) {
                        app.room_contacts.contacts.push(contact);
                    }
                }
                Command::SendToPeer | Command::SendToRoom => {
                    app.client
                        .send(&request)
                        .await
                        .map_err(std::io::Error::other)?;
                }
                _ => {}
            }

            app.textarea.clear();
        } else {
            app.textarea.input(key);
        }
    }
    Ok(Action::Continue)
}

async fn handle_event_login(app: &mut App, event: Event) -> io::Result<Action> {
    if let Event::Key(key) = event {
        if key.code == KeyCode::Enter {
            let username = app.login_textarea.lines().join("\n").trim().to_string();
            if !username.is_empty() {
                let username = username.replace(" ", "-");
                app.client
                    .login(&username)
                    .await
                    .map_err(std::io::Error::other)?;
                app.username = Some(username);
                app.logged_in = true;
                app.focus = Focus::PeerContactsArea;
            }
        } else {
            app.login_textarea.input(key);
        }
    }
    Ok(Action::Continue)
}

pub async fn update_state(app: &mut App) -> io::Result<Action> {
    let recorded_event = event::read()?;

    match recorded_event {
        Event::Mouse(mouse) => match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                app.shift_focus(Position::new(mouse.column, mouse.row));
            }
            _ => {}
        },

        _ => {}
    };

    match app.focus {
        Focus::Login => handle_event_login(app, recorded_event).await?,
        Focus::PeerContactsArea => handle_event_peer_contact_area(app, recorded_event)?,
        Focus::RoomContactsArea => handle_event_room_contact_area(app, recorded_event)?,
        Focus::ChatArea => handle_event_chat_area(app, recorded_event)?,
        Focus::InputBox => handle_event_input_box_area(app, recorded_event).await?,
    };

    Ok(Action::Continue)
}
