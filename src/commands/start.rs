use frankenstein::Message;

pub fn execute(bot: &crate::Bot, message: Message) {
    bot.send_message(message.chat.id, "Start message!");
}
