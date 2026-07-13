// handles events

use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use ratatui::layout::Position;
use crate::{app::{App, Focus}, events::Action::Continue};

pub enum Action {
    Continue,
    Quit,
}

pub fn update_state(app: &mut App) -> io::Result<Action> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            match key.code {
                KeyCode::Esc => return Ok(Action::Quit),
                KeyCode::Down => {
                    match app.focus {
                        Focus::PeerContactsArea => app.peer_contacts.select_next(),
                        Focus::RoomContactsArea => app.room_contacts.select_next(),
                        _ => {}
                    }
                },
                KeyCode::Up => {
                    match app.focus {
                        Focus::PeerContactsArea => app.peer_contacts.select_prev(),
                        Focus::RoomContactsArea => app.room_contacts.select_prev(),
                        _ => {}
                    }
                },
                _ => {}
            }
        },

        Event::Mouse(mouse)  => {
            match mouse.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    app.shift_focus(Position::new(mouse.column, mouse.row));
                    
                    match app.focus {
                        Focus::PeerContactsArea => {
                            let index = mouse.row - app.peer_contact_list_area.y - 1;
                            app.peer_contacts.select(Some(index as usize));
                            app.room_contacts.select(None);
                        },
                        Focus::RoomContactsArea => {
                            let index = mouse.row - app.room_contact_list_area.y - 1;
                            app.room_contacts.select(Some(index as usize));
                            app.peer_contacts.select(None);
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        _ => {}
    };

    Ok(Continue)
}