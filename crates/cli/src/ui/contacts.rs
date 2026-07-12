use ratatui::widgets::{ListItem, ListState};

pub enum ContactType {
    Peer,
    Room,
}

pub struct Contact {
    pub name: String,
    pub contact_type: ContactType,
}

impl Contact {
    pub fn new(name: impl Into<String>, contact_type: ContactType) -> Self {
        Self {
            name: name.into(),
            contact_type,
        }
    }
}

pub struct Contacts {
    pub contacts: Vec<Contact>,
    pub state: ListState,
}

impl Contacts {
    pub fn new(contacts: Vec<Contact>) -> Self {
        let mut list_state = ListState::default();
        if contacts.len() > 0 {
            list_state.select(Some(0));
        }
        Contacts {
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
            Contact::new("Homies", ContactType::Room),
            Contact::new("Fam", ContactType::Room),
        ])
    }

    pub fn len(&self) -> usize {
        self.contacts.len()
    }

    pub fn select_prev(&mut self) {
        if let Some(curr) = self.state.selected() {
            if self.contacts.is_empty() {
                return;
            }
            let len = self.len();
            self.state.select(Some((curr + len - 1) % len));
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn select_next(&mut self) {
        if let Some(curr) = self.state.selected() {
            if self.contacts.is_empty() {
                return;
            }
            let len = self.len();
            self.state.select(Some((curr + 1) % len));
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
