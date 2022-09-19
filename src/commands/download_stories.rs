use crate::{
    download::{self, ContentType},
    messages, utils,
};
use frankenstein::Message;
use std::fs;

pub async fn execute(bot: &crate::Bot, message: Message) {
    let user = match utils::get_content(&message) {
        Ok(res) => res,
        Err(_) => {
            messages::send_message(
                &bot.api,
                message.chat.id,
                "Incorrect usage of download-stories. See /help for assistance!",
            );

            return;
        }
    };

    let progress_msg =
        messages::send_message(&bot.api, message.chat.id, "⏳Searching the user stories...");

    let result = match download::fetch_content(&bot.client, ContentType::Stories(user)).await {
        Ok(result) => result,
        Err(text) => {
            messages::delete_message(&bot.api, progress_msg);
            messages::send_message(&bot.api, message.chat.id, &text);

            return;
        }
    };

    messages::edit_message(&bot.api, &progress_msg, "Start sending the stories...");

    let result = download::download_contents(&bot.client, result, message.chat.id).await;

    let (root_folder, files) = match result {
        Ok(paths) => paths,
        Err(err) => {
            eprintln!("Error while executing download contents: {}", err);

            messages::delete_message(&bot.api, progress_msg);
            messages::send_message(
                &bot.api,
                message.chat.id,
                "Something went wrong while downloading the stories. Please try again later!",
            );

            return;
        }
    };

    if let Err(_) = messages::send_medias(&bot.api, message.chat.id, files) {
        messages::send_message(
            &bot.api,
            message.chat.id,
            "Something went wrong while sending the stories. Please try again later!",
        );

        return;
    }

    messages::send_message(&bot.api, message.chat.id, "Finished!");

    fs::remove_dir_all(root_folder).unwrap_or_else(|e| {
        eprintln!("Error while deleting folder: {}", e);
    });
}
