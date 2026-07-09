pub enum Command {
    SetName,
    SendToPeer,
    SendToRoom,
    Unknown,
}

pub struct Request {
    pub command: Command,
    pub target: String,
    pub message: String,
}

impl Request {
    pub fn new(command: Command, target: String, message: String) -> Self {
        Self {
            command,
            target,
            message,
        }
    }

    pub fn from(text: String) -> Result<Self, String> {
        let words: Vec<&str> = text.split_whitespace().collect();

        if words.is_empty() {
            return Err("Empty request".to_string());
        }

        let command = match words[0] {
            "/name" => Command::SetName,
            "/msg" => Command::SendToPeer,
            "/shout" => Command::SendToRoom,
            _ => Command::Unknown,
        };

        match command {
            Command::SetName => {
                if words.len() != 2 {
                    return Err(String::from("Usage: /name <username>"));
                }

                Ok(Self::new(command, words[1].to_string(), String::new()))
            }

            Command::SendToPeer | Command::SendToRoom => {
                if words.len() < 3 {
                    return Err(String::from(
                        "Unsupported message format. Suggested format: \"<command> <target> <message>\"",
                    ));
                }

                Ok(Self::new(
                    command,
                    words[1].to_string(),
                    words[2..].join(" "),
                ))
            }

            Command::Unknown => Err(String::from("Unknown Command")),
        }
    }

    pub fn to_string(&self) -> Result<String, String> {
        let command = match self.command {
            Command::SetName => "/name",
            Command::SendToPeer => "/msg",
            Command::SendToRoom => "/shout",
            Command::Unknown => return Err(String::from("Unknown Command")),
        };

        Ok(format!("{} {} {}", command, self.target, self.message))
    }
}
