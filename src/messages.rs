use frankenstein::{
    Api, DeleteMessageParams, EditMessageTextParams, Message, SendMessageParams, SendPhotoParams,
    SendVideoParams, TelegramApi,
};
use std::{path::PathBuf, process};

pub fn send_message(api: &Api, chat_id: i64, text: &str) -> Message {
    let send_message_params = SendMessageParams::builder()
        .chat_id(chat_id)
        .text(text)
        .build();

    api.send_message(&send_message_params)
        .unwrap_or_else(|err| {
            eprintln!("Failed to send message: {}", err);

            process::exit(1);
        })
        .result
}

pub fn send_photo(api: &Api, chat_id: i64, file_path: PathBuf) -> Result<(), ()> {
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

pub fn send_video(api: &Api, chat_id: i64, file_path: PathBuf) -> Result<(), ()> {
    let send_video_params = SendVideoParams::builder()
        .chat_id(chat_id)
        .video(file_path)
        .build();

    if let Err(err) = api.send_video(&send_video_params) {
        eprintln!("Failed to send video: {}", err);

        return Err(());
    }

    Ok(())
}

pub fn send_medias(api: &Api, chat_id: i64, files: Vec<PathBuf>) -> Result<(), ()> {
    for file in files {
        let result;
        let extension = file.extension().unwrap();

        if extension == "jpeg" {
            result = send_photo(api, chat_id, file);
        } else {
            result = send_video(api, chat_id, file);
        }

        if let Err(_) = result {
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
        eprintln!("Failed to edit message: {}", err);
    }
}

pub fn delete_message(api: &Api, message: Message) {
    let delete_message_params = DeleteMessageParams::builder()
        .chat_id(message.chat.id)
        .message_id(message.message_id)
        .build();

    if let Err(err) = api.delete_message(&delete_message_params) {
        eprintln!("Failed to delete message: {}", err);
    }
}
