use crate::{
    commands::CommandInformation,
    messages::{self, ReplyWithKeyboard},
};
use frankenstein::InlineKeyboardButton;

pub fn execute(bot: &crate::Bot, command_information: &CommandInformation) {
    let text = &command_information.text;

    let keyboard = vec![
        vec![InlineKeyboardButton::builder()
            .text("Yes, download this post!!!")
            .callback_data(format!("/download_post {text}"))
            .build()],
        vec![InlineKeyboardButton::builder()
            .text("Yes, download the stories of this user!!!")
            .callback_data(format!("/download_stories {text}"))
            .build()],
        vec![InlineKeyboardButton::builder()
            .text("No, it was a mistake!!!")
            .callback_data("/continue")
            .build()],
    ];

    messages::send_message(
        &bot.api,
        command_information.chat_id,
        "Do you wanna do something about this?",
        Some(ReplyWithKeyboard {
            keyboard,
            message_id: command_information
                .message_id
                .expect("Expect message id in question command"),
        }),
    );
}
