use crate::messages;
use frankenstein::Message;

pub fn execute(bot: &crate::Bot, message: Message) {
    messages::send_message(
        &bot.api,
        message.chat.id,
        "Hi, my name is YAID and I can download Instagram posts and stories! \
        It's simple, I only have three easy-to-use commands:
        ∙ /download_post <post-link>
        ∙ /download_stories <user-name>
        ∙ /help\n\
        You just have to type the command followed by the requested parameter when needed and the rest is my responsability, no need to worry! \
        But remember: I can't send to you posts and stories from private profiles.",
    );
}
