// run the LOOP
mod app;
mod contacts;
mod draw;
mod events;
mod render;
mod settings;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use std::env;

use crate::{
    app::App,
    events::{Action, update_state},
};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    execute!(io::stdout(), EnableMouseCapture)?;
    let mut terminal = ratatui::init();
    let args: Vec<String> = env::args().collect();
    let server_addr = {
        if args.len() < 2 {
            "127.0.0.1:8080"
        } else {
            &args[1]
        }
    };

    let mut app = App::new(server_addr).await?;

    loop {
        match update_state(&mut app).await? {
            Action::Continue => {}
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
