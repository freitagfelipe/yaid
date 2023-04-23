use crate::{commands::CommandInformation, messages};

pub fn execute(bot: &crate::Bot, command_information: &CommandInformation) {
    messages::send_message(
        &bot.api,
        command_information.chat_id,
        "No problem! You can see /help if you need any assistance.",
        None,
    );
}
