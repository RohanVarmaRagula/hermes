// state of the ui

use crate::contacts::{PeerContacts, RoomContacts};
use client::Client;
use ratatui::layout::{Position, Rect};
use ratatui_textarea::TextArea;
use std::io::Result;

#[derive(PartialEq)]
pub enum Focus {
    Login,
    PeerContactsArea,
    RoomContactsArea,
    ChatArea,
    InputBox,
}

pub struct App {
    pub logged_in: bool,
    pub username: Option<String>,
    pub login_textarea: TextArea<'static>,
    pub focus: Focus,

    pub client: Client,

    pub peer_contacts: PeerContacts,
    pub peer_contact_list_area: Rect,

    pub room_contacts: RoomContacts,
    pub room_contact_list_area: Rect,

    pub chat_area: Rect,

    pub input_box_area: Rect,
    pub textarea: TextArea<'static>,
}

impl App {
    pub async fn new(server_addr: &str) -> Result<Self> {
        Ok(App {
            logged_in: false,
            username: None,
            login_textarea: TextArea::default(),
            focus: Focus::Login,
            client: Client::connect(server_addr).await?,
            peer_contacts: PeerContacts::example(),
            room_contacts: RoomContacts::example(),
            peer_contact_list_area: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            room_contact_list_area: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            chat_area: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            input_box_area: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            textarea: TextArea::default(),
        })
    }

    pub fn set_peer_contact_list_area(&mut self, peer_contact_list_area: Rect) {
        self.peer_contact_list_area = peer_contact_list_area
    }
    pub fn set_room_contact_list_area(&mut self, room_contact_list_area: Rect) {
        self.room_contact_list_area = room_contact_list_area
    }
    pub fn set_chat_area(&mut self, chat_area: Rect) {
        self.chat_area = chat_area
    }
    pub fn set_input_box_area(&mut self, input_box_area: Rect) {
        self.input_box_area = input_box_area
    }

    pub fn shift_focus(&mut self, position: Position) {
        if self.peer_contact_list_area.contains(position) {
            self.focus = Focus::PeerContactsArea;
        } else if self.room_contact_list_area.contains(position) {
            self.focus = Focus::RoomContactsArea;
        } else if self.chat_area.contains(position) {
            self.focus = Focus::ChatArea;
        } else if self.input_box_area.contains(position) {
            self.focus = Focus::InputBox;
        }
    }
}
