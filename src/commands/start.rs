use frankenstein::Message;

pub fn execute(bot: &crate::Bot, message: Message) {
    bot.send_message(
        message.chat.id,
        "Hi, my name is YAID and I can download Instagram posts! \
        It's simple, you just have to type /download <post_url> and the rest is my responsability, no need to worry! \
        But remember: I can't send to you posts from a private profile.",
    );
}
