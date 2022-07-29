mod commands;

use frankenstein::{AllowedUpdate, Error, UpdateContent};
use frankenstein::{Api, GetUpdatesParams, SendMessageParams, TelegramApi};
use reqwest::Client;
use tokio;

#[macro_export]
macro_rules! error {
    () => {
        Err("Something goes wrong! Plese, try again later!")
    };
    ($r:expr) => {
        Err($r)
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

                        if let Ok(command) = commands::has_valid_command(&message) {
                            tokio::spawn(async move {
                                commands::execute_command(&self.api, &self.client, command, message)
                                    .await
                            });
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
    
        if let Err(error) = self.api.send_message(&send_message_params) {
            panic!("Failed to send message: {:?}", error);
        }
    }
}
