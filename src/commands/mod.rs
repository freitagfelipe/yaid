mod continue_chat;
mod download_post;
mod download_stories;
mod help;
mod question;
mod start;

use frankenstein::{CallbackQuery, Message};

pub enum Command {
    Start,
    DowloadPost,
    DownloadStories,
    Question,
    Continue,
    Help,
}

pub struct CommandInformation {
    pub chat_id: i64,
    pub message_id: Option<i32>,
    pub text: String,
}

impl Command {
    pub fn from_message(message: &Message) -> Option<Self> {
        let text = message.text.as_ref()?;

        let command = text
            .split(' ')
            .next()
            .expect("Failed while getting the command");

        match command {
            "/start" => Some(Command::Start),
            "/download_post" => Some(Command::DowloadPost),
            "/download_stories" => Some(Command::DownloadStories),
            "/help" => Some(Command::Help),
            _ => Some(Command::Question),
        }
    }

    pub fn from_callback_query(callback_query: &CallbackQuery) -> Option<Self> {
        let command = callback_query
            .data
            .as_ref()
            .expect("Expect data in callback query")
            .as_str();

        match command {
            "/download_post" => Some(Command::DowloadPost),
            "/download_stories" => Some(Command::DownloadStories),
            "/continue" => Some(Command::Continue),
            _ => None,
        }
    }

    pub async fn execute(self, bot: &crate::Bot, command_information: &CommandInformation) {
        match self {
            Command::Start => start::execute(bot, command_information),
            Command::DowloadPost => download_post::execute(bot, command_information).await,
            Command::DownloadStories => download_stories::execute(bot, command_information).await,
            Command::Question => question::execute(bot, command_information),
            Command::Continue => continue_chat::execute(bot, command_information),
            Command::Help => help::execute(bot, command_information),
        };
    }
}
