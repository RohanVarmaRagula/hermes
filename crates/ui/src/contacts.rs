// handles contacts

use ratatui::widgets::{ListItem, ListState};

#[derive(PartialEq)]
pub enum ContactType {
    Peer,
    Room,
}

#[derive(PartialEq)]
pub struct Contact {
    pub name: String,
    pub contact_type: ContactType,
}

impl Contact {
    pub fn new(name: impl Into<String>, contact_type: ContactType) -> Self {
        Self {
            name: name.into(),
            contact_type: contact_type,
        }
    }
}

pub struct PeerContacts {
    pub contacts: Vec<Contact>,
    pub state: ListState,
}

pub struct RoomContacts {
    pub contacts: Vec<Contact>,
    pub state: ListState,
}

impl PeerContacts {
    pub fn new(contacts: Vec<Contact>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(None);
        PeerContacts {
            contacts,
            state: list_state,
        }
    }

    pub fn example() -> Self {
        Self::new(vec![
            Contact::new("Alice", ContactType::Peer),
            Contact::new("Bob", ContactType::Peer),
            Contact::new("Chud", ContactType::Peer),
            Contact::new("Dawg", ContactType::Peer),
        ])
    }

    pub fn len(&self) -> usize {
        self.contacts.len()
    }

    pub fn selected(&self) -> Option<&Contact> {
        if let Some(ind) = self.state.selected() {
            Some(&self.contacts[ind])
        } else {
            None
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        if let Some(i) = index {
            if i < self.len() {
                self.state.select(index);
            } else {
                self.state.select(None);
            }
        } else {
            self.state.select(None);
        }
    }

    pub fn select_prev(&mut self) {
        let len = self.len();
        if let Some(curr) = self.state.selected() {
            if len > 0 {
                self.state.select(Some((curr + len - 1) % len));
            }
        } else if len > 0 {
            self.state.select(Some(len - 1));
        }
    }

    pub fn select_next(&mut self) {
        let len = self.len();
        if let Some(curr) = self.state.selected() {
            if len > 0 {
                self.state.select(Some((curr + 1) % len));
            }
        } else if len > 0 {
            self.state.select(Some(0));
        }
    }

    pub fn items(&self) -> Vec<ListItem<'static>> {
        self.contacts
            .iter()
            .map(|contact| ListItem::new(contact.name.clone()))
            .collect()
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }
}

impl RoomContacts {
    pub fn new(contacts: Vec<Contact>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(None);
        RoomContacts {
            contacts,
            state: list_state,
        }
    }

    pub fn example() -> Self {
        Self::new(vec![
            Contact::new("Dawgs", ContactType::Room),
            Contact::new("Hbs", ContactType::Room),
            Contact::new("Kings House", ContactType::Room),
        ])
    }

    pub fn len(&self) -> usize {
        self.contacts.len()
    }

    pub fn selected(&self) -> Option<&Contact> {
        if let Some(ind) = self.state.selected() {
            Some(&self.contacts[ind])
        } else {
            None
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        if let Some(i) = index {
            if i < self.len() {
                self.state.select(index);
            } else {
                self.state.select(None);
            }
        } else {
            self.state.select(None);
        }
    }

    pub fn select_prev(&mut self) {
        let len = self.len();
        if let Some(curr) = self.state.selected() {
            if len > 0 {
                self.state.select(Some((curr + len - 1) % len));
            }
        } else if len > 0 {
            self.state.select(Some(len - 1));
        }
    }

    pub fn select_next(&mut self) {
        let len = self.len();
        if let Some(curr) = self.state.selected() {
            if len > 0 {
                self.state.select(Some((curr + 1) % len));
            }
        } else if len > 0 {
            self.state.select(Some(0));
        }
    }

    pub fn items(&self) -> Vec<ListItem<'static>> {
        self.contacts
            .iter()
            .map(|contact| ListItem::new(contact.name.clone()))
            .collect()
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }
}
