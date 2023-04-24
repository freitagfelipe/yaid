mod commands;
mod download;
mod handlers;
mod macros;
mod messages;
mod utils;
mod waitlist;

use commands::Command;
use frankenstein::{AllowedUpdate, Api, Error, GetUpdatesParams, TelegramApi, UpdateContent};
use reqwest::Client;
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
        let update_params_builder = GetUpdatesParams::builder()
            .allowed_updates(vec![AllowedUpdate::Message, AllowedUpdate::CallbackQuery]);
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

                let needed_information = match update.content {
                    UpdateContent::Message(message) => handlers::handle_message_update(&message),
                    UpdateContent::CallbackQuery(callback) => {
                        handlers::handle_callback_query_update(self, &callback)
                    }
                    _ => unreachable!(),
                };

                let Some((command, information, from_bot)) = needed_information else {
                    continue;
                };

                if from_bot {
                    continue;
                }

                if let Some(command) = command {
                    match command {
                        Command::DowloadPost | Command::DownloadStories => {
                            if !download_waitlist.add_to_waitlist(information.chat_id) {
                                messages::send_message(
                                    &self.api,
                                    information.chat_id,
                                    "I am already executing a download, \
                                    please wait it finish before running another!",
                                    None,
                                );

                                continue;
                            }
                        }
                        _ => {}
                    }

                    let mut waitlist = download_waitlist.clone();

                    tokio::spawn(async move {
                        command.execute(self, &information).await;
                        waitlist.remove_from_waitlist(information.chat_id);
                    });
                }
            }
        }
    }
}
