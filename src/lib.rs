mod commands;

use commands::Command;
use frankenstein::{AllowedUpdate, Error, UpdateContent};
use frankenstein::{
    Api, GetUpdatesParams, SendMessageParams, SendPhotoParams, SendVideoParams, TelegramApi,
};
use reqwest::Client;
use std::path::PathBuf;
use tokio;

#[macro_export]
macro_rules! error {
    (@reason $e:expr) => {
        eprintln!("Error: {}", $e);
    };
    ($e:expr, $b:expr $(, $o:expr)?) => {
        {
            if $b {
                error!(@reason $e);

                return  Err("Something went wrong! Please, try again later!".to_string());
            } else {
                $(error!(@reason $o))?

                return Err($e);
            }
        }
    };
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
            let result = self.api.get_updates(&update_params);

            match result {
                Ok(response) => {
                    for update in response.result {
                        let message = match update.content {
                            UpdateContent::Message(message) => message,
                            _ => unreachable!(),
                        };

                        if let Ok(command) = Command::new(&message) {
                            tokio::spawn(async move { command.execute(&self, message).await });
                        }

                        update_params = update_params_builder
                            .clone()
                            .offset(update.update_id + 1)
                            .build();
                    }
                }
                Err(error) => panic!("Failed to get updates: {:?}", error),
            }
        }
    }

    fn send_message(&self, chat_id: i64, text: &str) {
        let send_message_params = SendMessageParams::builder()
            .chat_id(chat_id)
            .text(text)
            .build();

        if let Err(err) = self.api.send_message(&send_message_params) {
            panic!("Failed to send message: {}", err);
        }
    }

    fn send_photo(&self, chat_id: i64, file_path: PathBuf) {
        let send_photo_params = SendPhotoParams::builder()
            .chat_id(chat_id)
            .photo(file_path)
            .build();

        if let Err(err) = self.api.send_photo(&send_photo_params) {
            eprint!("Failed to send photo: {}", err);

            self.send_message(chat_id, "Something goes wrong! Please try again later!");
        }
    }

    fn send_video(&self, chat_id: i64, file_path: PathBuf) {
        let send_video_params = SendVideoParams::builder()
            .chat_id(chat_id)
            .video(file_path)
            .build();

        if let Err(err) = self.api.send_video(&send_video_params) {
            eprint!("Failed to send photo: {}", err);

            self.send_message(chat_id, "Something goes wrong! Please try again later!");
        }
    }
}
