use frankenstein::Message;

pub fn execute(bot: &crate::Bot, message: Message) {
    bot.send_message(
        message.chat.id,
        "I have just one command, /download! \
        And it's really simple to use: type /download <post_url>, \
        and remember that I can't send to you posts from private profiles!",
    );
}
