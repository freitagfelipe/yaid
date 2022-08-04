use frankenstein::Message;

pub fn execute(bot: &crate::Bot, message: Message) {
    bot.send_message(
        message.chat.id,
        "I have just three commands:
        ∙ /download_post <post_link>
        ∙ /download_stories <user>
        ∙ /help\n\
        And it's really simple to use: type the command followed by the requested parameters when needed, \
        and remember that I can't send to you posts and stories from private profiles!",
    );
}
