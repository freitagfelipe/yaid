use frankenstein::{Api, Message, TelegramApi};
use frankenstein::SendMessageParams;

fn send_message(api: &Api, message: &Message, text: &str) {
    let send_message_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text(text)
        .build();

    if let Err(error) = api.send_message(&send_message_params) {
        panic!("Failed to send message: {:?}", error);
    }
}

fn start(api: &Api, message: Message) {
    // Todo start command for /start

    send_message(api, &message, "Start message!");
}

fn download(api: &Api, message: Message) {
    // Todo download command for /download url

    send_message(api, &message, "Download message!");
}

fn help(api: &Api, message: Message) {
    // Todo help command for /help

    send_message(api, &message, "Help message!");
}

pub fn has_valid_command(message: &Message) -> Result<Box<dyn FnOnce(&Api, Message) + Send>, ()> {
    let text = match message.text.as_ref() {
        Some(text) => text,
        None => return Err(())
    };
    
    let message_content = text.split(" ").collect::<Vec<&str>>()[0];

    match message_content {
        "/start" => Ok(Box::new(start)),
        "/download" => Ok(Box::new(download)),
        "/help" => Ok(Box::new(help)),
        _ => Err(())
    }
}
