mod download_post;
mod download_stories;
mod help;
mod start;

use frankenstein::Message;

pub enum Command {
    Start,
    DowloadPost,
    DownloadStories,
    Help,
}

impl Command {
    pub fn new(message: &Message) -> Result<Self, ()> {
        let text = message.text.as_ref().ok_or(())?;

        let command = text.split(" ").collect::<Vec<&str>>()[0];

        match command {
            "/start" => Ok(Command::Start),
            "/download_post" => Ok(Command::DowloadPost),
            "/download_stories" => Ok(Command::DownloadStories),
            "/help" => Ok(Command::Help),
            _ => Err(()),
        }
    }

    pub async fn execute(self, bot: &crate::Bot, message: Message) {
        match self {
            Command::Start => start::execute(bot, message),
            Command::DowloadPost => download_post::execute(bot, message).await,
            Command::DownloadStories => download_stories::execute(bot, message).await,
            Command::Help => help::execute(bot, message),
        };
    }
}
