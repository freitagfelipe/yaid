use crate::{
    commands::{Command, CommandInformation},
    messages, Bot,
};
use frankenstein::{CallbackQuery, Message};

type IsBot = bool;
type NeededInfo = (Option<Command>, CommandInformation, IsBot);

pub fn handle_message_update(message: &Message) -> Option<NeededInfo> {
    let message_text = message.text.as_ref()?;

    Some((
        Command::from_message(message),
        CommandInformation {
            chat_id: message.chat.id,
            message_id: Some(message.message_id),
            text: message_text.to_string(),
        },
        message
            .from
            .as_ref()
            .expect("Expect from in message")
            .is_bot,
    ))
}

pub fn handle_callback_query_update(bot: &Bot, callback: &CallbackQuery) -> Option<NeededInfo> {
    let message = callback
        .message
        .as_ref()
        .expect("Expect a message that originated the callback query");

    let mounted_text = format!(
        "{} {}",
        callback
            .data
            .as_ref()
            .expect("Expect data in callback query"),
        callback
            .message
            .as_ref()
            .expect("Expect message in callback query")
            .reply_to_message
            .as_ref()
            .expect("Expect a replied message")
            .text
            .as_ref()
            .expect("Expect text in replied message")
    );

    messages::remove_keyboard(&bot.api, message);

    Some((
        Command::from_callback_query(callback),
        CommandInformation {
            chat_id: message.chat.id,
            message_id: None,
            text: mounted_text,
        },
        false,
    ))
}
