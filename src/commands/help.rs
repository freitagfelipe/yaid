use crate::{commands::CommandInformation, messages};

pub fn execute(bot: &crate::Bot, command_information: &CommandInformation) {
    messages::send_message(
        &bot.api,
        command_information.chat_id,
        "I have just three commands:
        ∙ /download_post <post-link>
        ∙ /download_stories <user-name>
        ∙ /help\n\
        And it is really simple to use: type the command followed by the requested parameters when needed. \
        But if you do not want to type the hole command you can just send a message with the post link or user name and I will handle that. \
        Remember that I can not send to you posts and stories from private profiles and that I can not download stories from \
        a profile that did not post any stories today!",
        None
    );
}
