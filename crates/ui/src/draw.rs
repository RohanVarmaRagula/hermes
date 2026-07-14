// draws the ui

use crate::{
    app::{App, Focus},
    render::*,
    settings::*,
};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::List,
};

pub fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    if !app.logged_in {
        render_login(frame, frame.area(), app);
        return;
    }
    // Layout
    let [left, right] = Layout::horizontal([
        Constraint::Percentage(CONTACTS_HORIZONTAL_PERCENTAGE),
        Constraint::Percentage(CHAT_AREA_HORIZONTAL_PERCENTAGE),
    ])
    .areas(frame.area());

    let [peer_contact_list_area, room_contact_list_area] =
        Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(left);

    let [chat_area, input_box_area] = Layout::vertical([
        Constraint::Percentage(CHAT_AREA_VERTICAL_PERCENTAGE),
        Constraint::Percentage(INPUT_BOX_VERTICAL_PERCENTAGE),
    ])
    .areas(right);

    // set state
    app.set_peer_contact_list_area(peer_contact_list_area);
    app.set_room_contact_list_area(room_contact_list_area);
    app.set_chat_area(chat_area);
    app.set_input_box_area(input_box_area);

    // widgets: left
    // widgets: left // contacts

    let peer_list = List::new(app.peer_contacts.items());
    let room_list = List::new(app.room_contacts.items());

    // widgets: rigth // chat
    // widgets: rigth // chat box

    // render

    render_peer_contacts(
        frame,
        peer_list,
        peer_contact_list_area,
        app,
        app.focus == Focus::PeerContactsArea,
    );

    render_room_contacts(
        frame,
        room_list,
        room_contact_list_area,
        app,
        app.focus == Focus::RoomContactsArea,
    );

    render_chat_area(frame, chat_area, app.focus == Focus::ChatArea);

    render_input_box(frame, input_box_area, app, app.focus == Focus::InputBox);
}
