use std::{
    fmt::{self, Display},
    str::FromStr,
};

pub enum Command {
    AddPeer,
    AddRoom,
    SetName,
    SendToPeer,
    SendToRoom,
    Unknown,
}

pub struct Request {
    pub command: Command,
    pub sender: String,
    pub target: String,
    pub message: String,
}

impl Request {
    pub fn new(command: Command, sender: String, target: String, message: String) -> Self {
        Self {
            command,
            sender,
            target,
            message,
        }
    }

    pub fn set_name(username: impl Into<String>) -> Self {
        Self::new(
            Command::SetName,
            String::new(),
            username.into(),
            String::new(),
        )
    }

    pub fn add_peer(target: impl Into<String>) -> Self {
        Self::new(
            Command::AddPeer,
            String::new(),
            target.into(),
            String::new(),
        )
    }

    pub fn add_room(target: impl Into<String>) -> Self {
        Self::new(
            Command::AddRoom,
            String::new(),
            target.into(),
            String::new(),
        )
    }

    pub fn send_to_peer(target: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(
            Command::SendToPeer,
            String::new(),
            target.into(),
            message.into(),
        )
    }

    pub fn send_to_room(room: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(
            Command::SendToRoom,
            String::new(),
            room.into(),
            message.into(),
        )
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let command = match self.command {
            Command::AddPeer => "/add_peer",
            Command::AddRoom => "/add_room",
            Command::SetName => "/name",
            Command::SendToPeer => "/msg",
            Command::SendToRoom => "/shout",
            Command::Unknown => return Err(fmt::Error),
        };

        write!(
            f,
            "{} {} {} {}",
            command, self.sender, self.target, self.message
        )
    }
}

impl FromStr for Request {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();

        if words.is_empty() {
            return Err("Empty request".to_string());
        }

        let command = match words[0] {
            "add_peer" => Command::AddPeer,
            "add_room" => Command::AddRoom,
            "/name" => Command::SetName,
            "/msg" => Command::SendToPeer,
            "/shout" => Command::SendToRoom,
            _ => Command::Unknown,
        };

        match command {
            Command::AddPeer => {
                if words.len() != 2 {
                    return Err("Usage: /add_peer <peername>".to_string());
                }

                Ok(Self::new(
                    Command::AddPeer,
                    String::new(),
                    words[1].to_string(),
                    String::new(),
                ))
            }

            Command::AddRoom => {
                if words.len() != 2 {
                    return Err("Usage: /add_roome <roomename>".to_string());
                }

                Ok(Self::new(
                    Command::AddRoom,
                    String::new(),
                    words[1].to_string(),
                    String::new(),
                ))
            }

            Command::SetName => {
                println!("{:?}", words);
                if words.len() != 2 {
                    return Err("Usage: /name <username>\n".to_string());
                }

                Ok(Self::set_name(words[1]))
            }

            Command::SendToPeer => {
                if words.len() < 4 {
                    return Err("Usage: /msg <sender> <target> <message>".to_string());
                }

                Ok(Self::new(
                    Command::SendToPeer,
                    words[1].to_string(),
                    words[2].to_string(),
                    words[3..].join(" "),
                ))
            }

            Command::SendToRoom => {
                if words.len() < 4 {
                    return Err("Usage: /shout <sender> <room> <message>".to_string());
                }

                Ok(Self::new(
                    Command::SendToRoom,
                    words[1].to_string(),
                    words[2].to_string(),
                    words[3..].join(" "),
                ))
            }

            Command::Unknown => Err("Unknown Command".to_string()),
        }
    }
}
