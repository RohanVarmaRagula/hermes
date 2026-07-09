use std::sync::Arc;
use tokio::{net::tcp::OwnedWriteHalf, sync::Mutex};

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
}

impl User {
    pub fn new(
        name: String, 
        writer: Arc<Mutex<OwnedWriteHalf>>
    ) -> Self {
        Self { name, writer }
    }
}