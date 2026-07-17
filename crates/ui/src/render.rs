// renders each widget

use crate::{app::App, settings::*};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Styled, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, List, Paragraph},
};

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
                        Line::from("Friends")
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
                        Line::from("Rooms")
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

pub fn render_input_box(frame: &mut Frame, area: Rect, app: &mut App, highlight: bool) {
    let border = if highlight { ACTIVE } else { BORDER };

    app.textarea
        .set_block(Block::default().borders(Borders::ALL).border_style(border));

    frame.render_widget(&app.textarea, area);
}

pub fn render_login(frame: &mut Frame, area: Rect, app: &mut App) {
    // Outer bordered box with title
    let outer = Block::bordered()
        .title(
            Line::from("Hermes")
                .centered()
                .style(Style::default().fg(TITLE)),
        )
        .border_style(Style::default().fg(BORDER))
        .set_style(Style::default().bg(BG));

    frame.render_widget(outer, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(area);

    let prompt = Paragraph::new(Text::from(Line::from(
        "Enter your username (no whitespace)",
    )))
    .alignment(Alignment::Center)
    .style(Style::default().fg(TEXT).bg(BG));
    frame.render_widget(prompt, chunks[1]);

    // Input box: center horizontally
    let input_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(chunks[2]);

    app.login_textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ACTIVE)),
    );

    frame.render_widget(&app.login_textarea, input_cols[1]);

    let footer = Paragraph::new(Text::from(Line::from("Press Enter to continue")))
        .alignment(Alignment::Center)
        .style(Style::default().fg(TEXT).bg(BG));
    frame.render_widget(footer, chunks[4]);
}
