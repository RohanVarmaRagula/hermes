// runs the LOOP

use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute};

use crate::ui::{app::App, draw, events::{Action, update_state}};
use std::io;

pub fn run() -> io::Result<()> {
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
