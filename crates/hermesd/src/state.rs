use crate::user::User;
use std::{collections::HashMap, sync::Arc};
use tokio::{net::tcp::OwnedWriteHalf, sync::Mutex};

pub struct ServerState {
    users: HashMap<String, User>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn enroll_user(&mut self, user: User) -> Result<(), String> {
        if self.users.contains_key(&user.name) {
            return Err(format!("username `{}` already exists", user.name));
        }

        self.users.insert(user.name.clone(), user);
        Ok(())
    }

    pub fn remove_user(&mut self, name: &str) {
        self.users.remove(name);
    }

    pub fn get_writer(&self, name: &str) -> Result<Arc<Mutex<OwnedWriteHalf>>, String> {
        self.users
            .get(name)
            .map(|u| Arc::clone(&u.writer))
            .ok_or_else(|| format!("username `{name}` not found"))
    }

    pub fn has_user(&self, name: &str) -> bool {
        self.users.contains_key(name)
    }
}
