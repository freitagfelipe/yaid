mod download;
mod help;
mod start;

use frankenstein::Message;

pub enum Command {
    Start,
    Dowload,
    Help,
}

impl Command {
    pub fn new(message: &Message) -> Result<Self, ()> {
        let text = message.text.as_ref().ok_or(())?;

        let command = text.split(" ").collect::<Vec<&str>>()[0];

        match command {
            "/start" => Ok(Command::Start),
            "/download" => Ok(Command::Dowload),
            "/help" => Ok(Command::Help),
            _ => Err(()),
        }
    }

    pub async fn execute(self, bot: &crate::Bot, message: Message) {
        match self {
            Command::Start => start::execute(bot, message),
            Command::Dowload => download::execute(bot, message).await,
            Command::Help => help::execute(bot, message),
        };
    }
}
