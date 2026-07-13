// handles contacts

use ratatui::widgets::{ListItem, ListState};

pub struct Contact {
    pub name: String,
}

impl Contact {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
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
        if contacts.len() > 0 {
            list_state.select(Some(0));
        }
        PeerContacts {
            contacts,
            state: list_state,
        }
    }

    pub fn example() -> Self {
        Self::new(vec![
            Contact::new("Alice"),
            Contact::new("Bob"),
            Contact::new("Chud"),
            Contact::new("Dawg"),
        ])
    }

    pub fn len(&self) -> usize {
        self.contacts.len()
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
        if contacts.len() > 0 {
            list_state.select(Some(0));
        }
        RoomContacts {
            contacts,
            state: list_state,
        }
    }

    pub fn example() -> Self {
        Self::new(vec![
            Contact::new("Dawgs"),
            Contact::new("Hbs"),
            Contact::new("Kings House"),
        ])
    }

    pub fn len(&self) -> usize {
        self.contacts.len()
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
