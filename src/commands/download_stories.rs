use crate::{
    commands::CommandInformation,
    download::{self, ContentType},
    messages,
    utils::{self, Error},
};
use std::fs;

pub async fn execute(bot: &crate::Bot, command_information: &CommandInformation) {
    let user = match utils::get_content(&command_information.text) {
        Ok(res) => res,
        Err(Error::TooMuchParameters) => {
            messages::send_message(
                &bot.api,
                command_information.chat_id,
                "Incorrect usage of download-stories too much parameters. See /help for assistance!",
                None,
            );

            return;
        }
        Err(Error::NoSecondParameter) => {
            messages::send_message(
                &bot.api,
                command_information.chat_id,
                "Incorrect usage of download-stories missing username. See /help for assistance!",
                None,
            );

            return;
        }
    };

    let progress_msg = messages::send_message(
        &bot.api,
        command_information.chat_id,
        "⏳Searching the user stories...",
        None,
    );

    let result = match download::fetch_content(&bot.client, ContentType::Stories(user)).await {
        Ok(result) => result,
        Err(text) => {
            messages::delete_message(&bot.api, progress_msg);
            messages::send_message(&bot.api, command_information.chat_id, &text, None);

            return;
        }
    };

    messages::edit_message(&bot.api, &progress_msg, "Start sending the stories...");

    let result =
        download::download_contents(&bot.client, result, command_information.chat_id).await;

    let (root_folder, files) = match result {
        Ok(paths) => paths,
        Err(err) => {
            eprintln!("Error while executing download contents: {}", err);

            messages::delete_message(&bot.api, progress_msg);
            messages::send_message(
                &bot.api,
                command_information.chat_id,
                "Something went wrong while downloading the stories. Please try again later!",
                None,
            );

            return;
        }
    };

    if messages::send_medias(&bot.api, command_information.chat_id, files).is_err() {
        messages::send_message(
            &bot.api,
            command_information.chat_id,
            "Something went wrong while sending the stories. Please try again later!",
            None,
        );

        return;
    }

    messages::send_message(&bot.api, command_information.chat_id, "Finished!", None);

    if let Err(err) = fs::remove_dir_all(root_folder) {
        eprintln!("Error while deleting folder: {}", err);
    }
}
