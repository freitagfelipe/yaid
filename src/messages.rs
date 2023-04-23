use frankenstein::{
    Api, DeleteMessageParams, EditMessageReplyMarkupParams, EditMessageTextParams,
    InlineKeyboardButton, InlineKeyboardMarkup, Message, ReplyMarkup, SendMessageParams,
    SendPhotoParams, SendVideoParams, TelegramApi,
};
use std::{path::PathBuf, process};

pub struct ReplyWithKeyboard {
    pub keyboard: Vec<Vec<InlineKeyboardButton>>,
    pub message_id: i32,
}

pub fn send_message(
    api: &Api,
    chat_id: i64,
    text: &str,
    reply_with_keyboard: Option<ReplyWithKeyboard>,
) -> Message {
    let message_id = reply_with_keyboard.as_ref().map(|reply| reply.message_id);

    let keyboard_markup = reply_with_keyboard.map(|reply| {
        InlineKeyboardMarkup::builder()
            .inline_keyboard(reply.keyboard)
            .build()
    });

    let send_message_params_builder = SendMessageParams::builder();

    let send_message_params = match (keyboard_markup, message_id) {
        (Some(markup), Some(message_id)) => send_message_params_builder
            .reply_to_message_id(message_id)
            .chat_id(chat_id)
            .text(text)
            .reply_markup(ReplyMarkup::InlineKeyboardMarkup(markup))
            .build(),
        (None, None) => send_message_params_builder
            .text(text)
            .chat_id(chat_id)
            .build(),
        _ => unreachable!(),
    };

    api.send_message(&send_message_params)
        .unwrap_or_else(|err| {
            eprintln!("Failed to send message: {}", err);

            process::exit(1);
        })
        .result
}

fn send_photo(api: &Api, chat_id: i64, file_path: PathBuf) -> Result<(), ()> {
    let send_photo_params = SendPhotoParams::builder()
        .chat_id(chat_id)
        .photo(file_path)
        .build();

    if let Err(err) = api.send_photo(&send_photo_params) {
        eprintln!("Failed to send photo: {}", err);

        return Err(());
    }

    Ok(())
}

fn send_video(api: &Api, chat_id: i64, file_path: PathBuf) -> Result<(), ()> {
    let send_video_params = SendVideoParams::builder()
        .chat_id(chat_id)
        .video(file_path)
        .build();

    if let Err(err) = api.send_video(&send_video_params) {
        eprintln!("Failed to send video: {err}");

        return Err(());
    }

    Ok(())
}

pub fn send_medias(api: &Api, chat_id: i64, files: Vec<PathBuf>) -> Result<(), ()> {
    for file in files {
        let extension = file
            .extension()
            .expect("Failed while getting the file extension");

        let result = if extension == "jpeg" {
            send_photo(api, chat_id, file)
        } else {
            send_video(api, chat_id, file)
        };

        if result.is_err() {
            return Err(());
        }
    }

    Ok(())
}

pub fn edit_message(api: &Api, message: &Message, new_text: &str) {
    let edit_message_params = EditMessageTextParams::builder()
        .chat_id(message.chat.id)
        .text(new_text)
        .message_id(message.message_id)
        .build();

    if let Err(err) = api.edit_message_text(&edit_message_params) {
        eprintln!("Failed to edit message: {err}");
    }
}

pub fn remove_keyboard(api: &Api, message: &Message) {
    let edit_message_params = EditMessageReplyMarkupParams::builder()
        .chat_id(message.chat.id)
        .message_id(message.message_id)
        .build();

    if let Err(err) = api.edit_message_reply_markup(&edit_message_params) {
        eprintln!("Failed to remove markup: {err}")
    }
}

pub fn delete_message(api: &Api, message: Message) {
    let delete_message_params = DeleteMessageParams::builder()
        .chat_id(message.chat.id)
        .message_id(message.message_id)
        .build();

    if let Err(err) = api.delete_message(&delete_message_params) {
        eprintln!("Failed to delete message: {err}");
    }
}
