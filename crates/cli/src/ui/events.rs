use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::ui::{app::App, events::Action::Continue};

pub enum Action {
    Continue,
    Quit,
}

pub fn update_state(app: &mut App) -> io::Result<Action> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            let x = key.code;
            match x {
                KeyCode::Esc => return Ok(Action::Quit),
                KeyCode::Down => app.contacts.select_next(),
                KeyCode::Up => app.contacts.select_prev(),
                _ => {}
            }
        },

        _ => {}
    };

    Ok(Continue)
}