use crate::{
    commands::{Command, CommandInformation},
    messages, Bot,
};
use frankenstein::{CallbackQuery, Message};

type IsBot = bool;
type NeededInfo = (Option<Command>, CommandInformation, IsBot);

pub fn handle_message_update(message: &Message) -> NeededInfo {
    (
        Command::from_message(message),
        CommandInformation {
            chat_id: message.chat.id,
            message_id: Some(message.message_id),
            text: message
                .text
                .as_ref()
                .expect("Expect text in message")
                .to_string(),
        },
        message
            .from
            .as_ref()
            .expect("Expect from in message")
            .is_bot,
    )
}

pub fn handle_callback_query_update(bot: &Bot, callback: &CallbackQuery) -> NeededInfo {
    let message = callback
        .message
        .as_ref()
        .expect("Expect a message that originated the callback query");

    messages::remove_keyboard(&bot.api, message);

    (
        Command::from_callback_query(callback),
        CommandInformation {
            chat_id: message.chat.id,
            message_id: None,
            text: callback
                .data
                .as_ref()
                .expect("Expect data in callback query")
                .to_string(),
        },
        false,
    )
}
