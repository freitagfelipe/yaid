use crate::messages;
use frankenstein::Message;

pub fn execute(bot: &crate::Bot, message: Message) {
    messages::send_message(
        &bot.api,
        message.chat.id,
        "I have just three commands:
        ∙ /download_post <post-link>
        ∙ /download_stories <user-name>
        ∙ /help\n\
        And it is really simple to use: type the command followed by the requested parameters when needed, \
        and remember that I can not send to you posts and stories from private profiles! Also, remember that I can not download stories from \
        a profile that did not post any stories today!",
    );
}
