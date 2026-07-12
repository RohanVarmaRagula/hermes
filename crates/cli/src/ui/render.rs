use crate::ui::{app::App, draw, events::{Action, update_state}};
use std::io;

pub fn run() -> io::Result<()> {
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

    ratatui::restore();

    Ok(())
}
