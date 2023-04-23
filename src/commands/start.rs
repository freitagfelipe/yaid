use crate::{commands::CommandInformation, messages};

pub fn execute(bot: &crate::Bot, command_information: &CommandInformation) {
    messages::send_message(
        &bot.api,
        command_information.chat_id,
        "Hi, my name is YAID and I can download Instagram posts and stories! \
        It is simple, I only have three easy-to-use commands:
        ∙ /download_post <post-link>
        ∙ /download_stories <user-name>
        ∙ /help\n\
        You just have to type the command followed by the requested parameter when needed and the rest is my responsability, no need to worry! \
        But if you do not want to type the hole command you can just send a message with the post link or user name and I will handle that. \
        But remember: I can not send to you posts and stories from private profiles.",
        None
    );
}
