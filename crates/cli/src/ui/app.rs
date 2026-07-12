use crate::ui::contacts::Contacts;

pub struct App {
    pub contacts: Contacts,
}

impl App {
    pub fn new() -> Self {
        App {
            contacts: Contacts::example(),
        }
    }
}
