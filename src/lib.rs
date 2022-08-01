mod commands;

use commands::Command;
use frankenstein::{AllowedUpdate, Error, UpdateContent};
use frankenstein::{
    Api, DeleteMessageParams, EditMessageTextParams, GetUpdatesParams, Message, SendMessageParams,
    SendPhotoParams, SendVideoParams, TelegramApi,
};
use reqwest::Client;
use std::{path::PathBuf, process};
use tokio;

#[macro_export]
macro_rules! error {
    (@reason $r:expr) => {
        eprintln!("Error: {}", $r)
    };
    (@error $e:expr) => {
        return Err($e.to_string())
    };
    (r: $r:expr) => {
        {
            error!(@reason $r);

            error!(@error "Something went wrong! Please try again later!")
        }
    };
    (e: $e:expr $(, r: $r:expr )?) => {
        $( error!(@reason $r); )?

        error!(@error $e)
    }
}
pub struct Bot {
    api: Api,
    client: Client,
}

impl Bot {
    pub fn new(token: &str) -> Result<&'static Self, Error> {
        let api = Api::new(token);

        api.get_me()?;

        let client = Client::new();

        Ok(Box::leak(Box::new(Bot { api, client })))
    }

    pub fn get_updates(&'static self) {
        let update_params_builder =
            GetUpdatesParams::builder().allowed_updates(vec![AllowedUpdate::Message]);
        let mut update_params = update_params_builder.clone().build();

        loop {
            let response = match self.api.get_updates(&update_params) {
                Ok(response) => response,
                Err(err) => {
                    eprintln!("Failed to get updates: {}", err);

                    continue;
                }
            };

            for update in response.result {
                update_params = update_params_builder
                    .clone()
                    .offset(update.update_id + 1)
                    .build();

                let message = match update.content {
                    UpdateContent::Message(message) => message,
                    _ => unreachable!(),
                };

                if let Some(user) = message.from.as_ref() {
                    if user.is_bot {
                        continue;
                    }
                }

                if let Ok(command) = Command::new(&message) {
                    tokio::spawn(async move { command.execute(&self, message).await });
                }
            }
        }
    }

    fn send_message(&self, chat_id: i64, text: &str) -> Message {
        let send_message_params = SendMessageParams::builder()
            .chat_id(chat_id)
            .text(text)
            .build();

        self.api
            .send_message(&send_message_params)
            .unwrap_or_else(|err| {
                eprintln!("Failed to send message: {}", err);

                process::exit(1);
            })
            .result
    }

    fn send_photo(&self, chat_id: i64, file_path: PathBuf) {
        let send_photo_params = SendPhotoParams::builder()
            .chat_id(chat_id)
            .photo(file_path)
            .build();

        if let Err(err) = self.api.send_photo(&send_photo_params) {
            self.send_message(
                chat_id,
                "Something went wrong while sending your photo! Please try again later!",
            );

            panic!("Failed to send photo: {}", err);
        }
    }

    fn send_video(&self, chat_id: i64, file_path: PathBuf) {
        let send_video_params = SendVideoParams::builder()
            .chat_id(chat_id)
            .video(file_path)
            .build();

        if let Err(err) = self.api.send_video(&send_video_params) {
            self.send_message(
                chat_id,
                "Something went wrong while sending your video! Please try again later!",
            );

            panic!("Failed to send video: {}", err);
        }
    }

    fn edit_message(&self, message: &Message, new_text: &str) {
        let edit_message_params = EditMessageTextParams::builder()
            .chat_id(message.chat.id)
            .text(new_text)
            .message_id(message.message_id)
            .build();

        if let Err(err) = self.api.edit_message_text(&edit_message_params) {
            self.send_message(
                message.chat.id,
                "Something went wrong! Please try again later!",
            );

            panic!("Failed to edit message: {}", err);
        }
    }

    fn delete_message(&self, message: Message) {
        let delete_message_params = DeleteMessageParams::builder()
            .chat_id(message.chat.id)
            .message_id(message.message_id)
            .build();

        if let Err(err) = self.api.delete_message(&delete_message_params) {
            self.send_message(
                message.chat.id,
                "Something went wrong! Please try again later!",
            );

            panic!("Failed to delete message: {}", err);
        }
    }

    fn send_medias(&self, chat_id: i64, files: Vec<PathBuf>) {
        for file in files {
            let extension = file.extension().unwrap();

            if extension == "jpeg" {
                self.send_photo(chat_id, file);
            } else {
                self.send_video(chat_id, file);
            }
        }
    }
}
