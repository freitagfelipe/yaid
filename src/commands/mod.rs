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
    pub fn new(message: &Message) -> Option<Self> {
        let text = message.text.as_ref()?;

        let command = text
            .split(' ')
            .next()
            .expect("Failed while getting the command");

        match command {
            "/start" => Some(Command::Start),
            "/download_post" => Some(Command::DowloadPost),
            "/download_stories" => Some(Command::DownloadStories),
            _ => Some(Command::Help),
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
