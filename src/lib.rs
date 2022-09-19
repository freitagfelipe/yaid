mod commands;
mod download;
mod macros;
mod messages;
mod utils;
mod waitlist;

use commands::Command;
use frankenstein::{AllowedUpdate, Api, Error, GetUpdatesParams, TelegramApi, UpdateContent};
use reqwest::Client;
use tokio;
use waitlist::Waitlist;

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
        let mut download_waitlist = Waitlist::new();

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
                    let chat_id = message.chat.id;

                    match command {
                        Command::DowloadPost | Command::DownloadStories => {
                            if !download_waitlist.add_to_waitlist(chat_id) {
                                messages::send_message(
                                    &self.api,
                                    chat_id,
                                    "I'm already executing a download, \
                                    please wait it finish before running another!",
                                );

                                continue;
                            }
                        }
                        _ => (),
                    }

                    let mut waitlist = download_waitlist.clone();

                    tokio::spawn(async move {
                        command.execute(&self, message).await;
                        waitlist.remove_from_waitlist(chat_id);
                    });
                }
            }
        }
    }
}
