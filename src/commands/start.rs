use frankenstein::{Api, Message};

pub fn execute(api: &Api, message: Message) {
    crate::send_message(api, message.chat.id, "Start message!");
}
