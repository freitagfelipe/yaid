mod commands;

use frankenstein::{AllowedUpdate, Error, UpdateContent};
use frankenstein::{Api, GetUpdatesParams, TelegramApi};
use std::thread;
use crate::commands as cmd;

pub struct Bot {
    api: Api,
}

impl Bot {
    pub fn new(token: &str) -> Result<&'static Self, Error> {
        let api = Api::new(token);

        api.get_me()?;

        Ok(Box::leak(Box::new(Bot { api })))
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

                        if let Ok(command) = cmd::has_valid_command(&message) {
                            if let Err(_) = thread::Builder::new().spawn(move || command(&self.api, message)) {
                                // Todo thread spawn error handler
                            }
                        }

                        update_params = update_params_builder
                            .clone()
                            .offset(update.update_id + 1)
                            .build();
                    }
                }
                Err(error) => panic!("Failed to get updates: {:?}", error)
            }
        }
    }
}
