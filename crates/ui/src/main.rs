// run the LOOP
mod settings;
mod app;
mod render;
mod contacts;
mod draw;
mod events;

use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute};

use crate::{app::App, events::{Action, update_state}};
use std::io;

fn main() -> io::Result<()> {
    execute!(io::stdout(), EnableMouseCapture)?;
    let mut terminal = ratatui::init();

    let mut app = App::new();

    loop {
        match update_state(&mut app)? {
            Action::Continue => {},
            Action::Quit => break,
        }

        terminal.draw(|frame| {
            draw::draw(frame, &mut app);
        })?;
    }

    execute!(io::stdout(), DisableMouseCapture)?;
    ratatui::restore();

    Ok(())
}
