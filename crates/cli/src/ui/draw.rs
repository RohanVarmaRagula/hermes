use ratatui::{
    layout::{Constraint, Layout}, style::{Style, Styled}, text::Line, widgets::{Block, List},
};
use crate::ui::theme::{*};
use crate::ui::app::App;

pub fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    // Layout
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(20), Constraint::Fill(1)]).areas(frame.area());

    let [chat, input_box] =
        Layout::vertical([Constraint::Percentage(90), Constraint::Fill(1)]).areas(right);

    // widgets: left
    // widgets: left // contacts

    let list = List::new(app.contacts.items());

    // widgets: rigth // chat
    // widgets: rigth // chat box
    

    // render
    // render: left // contacts

    frame.render_stateful_widget(
        list
            .style(
                Style::default()
                    .fg(TEXT)
                    .bg(BG)
            )
            .highlight_style(
                Style::default()
                    .bg(SURFACE)
                    .fg(ACTIVE)
            )
            .highlight_symbol(">> ")
            .block(
                Block::bordered()
                    .title(
                        Line::from("Contacts")
                            .centered()
                            .style(
                                Style::default()
                                    .fg(TITLE)
                            )
                    )
                    .border_style(
                        Style::default()
                            .fg(BORDER)
                    )
                    .set_style(
                        Style::default()
                            .bg(BG)
                    )
            ),
        left, 
        app.contacts.state_mut()
    );
    
    // render: right // chat
    
    frame.render_widget(
        Block::bordered()
            .title(
                Line::from("Hermes")
                    .centered()
                    .style(
                        Style::default()
                            .fg(TITLE)
                    )
            )
            .border_style(
                Style::default()
                    .fg(BORDER)
            )
            .set_style(
                Style::default()
                    .bg(BG)
            ),
    chat,
    );

    // render: right // chat box
    
    frame.render_widget(
        Block::bordered()
            .title(
                Line::from("Type Here")
                    .centered()
                    .style(
                        Style::default()
                            .fg(TITLE)
                    )
            )
            .border_style(
                Style::default()
                    .fg(BORDER)
            )
            .set_style(
                Style::default()
                    .bg(BG)
            ),
        input_box,
    );
    
}
