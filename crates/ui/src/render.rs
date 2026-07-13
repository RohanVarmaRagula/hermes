// renders each widget

use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Styled},
    text::Line,
    widgets::{Block, List},
};

use crate::{app::App, settings::*};

pub fn render_peer_contacts(
    frame: &mut Frame,
    peer_list: List,
    area: Rect,
    app: &mut App,
    highlight: bool,
) {
    let border = if highlight { ACTIVE } else { BORDER };

    frame.render_stateful_widget(
        peer_list
            .style(Style::default().fg(TEXT).bg(BG))
            .highlight_style(Style::default().bg(SURFACE).fg(ACTIVE))
            .highlight_symbol(">> ")
            .block(
                Block::bordered()
                    .title(
                        Line::from("Contacts")
                            .centered()
                            .style(Style::default().fg(TITLE)),
                    )
                    .border_style(Style::default().fg(border))
                    .set_style(Style::default().bg(BG)),
            ),
        area,
        app.peer_contacts.state_mut(),
    );
}

pub fn render_room_contacts(
    frame: &mut Frame,
    room_list: List,
    area: Rect,
    app: &mut App,
    highlight: bool,
) {
    let border = if highlight { ACTIVE } else { BORDER };

    frame.render_stateful_widget(
        room_list
            .style(Style::default().fg(TEXT).bg(BG))
            .highlight_style(Style::default().bg(SURFACE).fg(ACTIVE))
            .highlight_symbol(">> ")
            .block(
                Block::bordered()
                    .title(
                        Line::from("Contacts")
                            .centered()
                            .style(Style::default().fg(TITLE)),
                    )
                    .border_style(Style::default().fg(border))
                    .set_style(Style::default().bg(BG)),
            ),
        area,
        app.room_contacts.state_mut(),
    );
}

pub fn render_chat_area(frame: &mut Frame, area: Rect, highlight: bool) {
    let border = if highlight { ACTIVE } else { BORDER };

    frame.render_widget(
        Block::bordered()
            .title(
                Line::from("Hermes")
                    .centered()
                    .style(Style::default().fg(TITLE)),
            )
            .border_style(Style::default().fg(border))
            .set_style(Style::default().bg(BG)),
        area,
    );
}

pub fn render_input_box(frame: &mut Frame, area: Rect, highlight: bool) {
    let border = if highlight { ACTIVE } else { BORDER };

    frame.render_widget(
        Block::bordered()
            .title(
                Line::from("Type Here")
                    .centered()
                    .style(Style::default().fg(TITLE)),
            )
            .border_style(Style::default().fg(border))
            .set_style(Style::default().bg(BG)),
        area,
    );
}
