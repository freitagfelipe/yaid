use crate::messages;
use crate::{
    download::{self, ContentType},
    utils,
};
use frankenstein::Message;
use std::fs;

pub async fn execute(bot: &crate::Bot, message: Message) {
    let post = match utils::get_content(&message) {
        Some(res) => res,
        None => {
            messages::send_message(
                &bot.api,
                message.chat.id,
                "Incorrect usage of download-post. See /help for assistance!",
            );

            return;
        }
    };

    let progress_msg = messages::send_message(&bot.api, message.chat.id, "⏳Searching the post...");

    let result = match download::fetch_content(&bot.client, ContentType::Post(post)).await {
        Ok(result) => result,
        Err(text) => {
            messages::delete_message(&bot.api, progress_msg);
            messages::send_message(&bot.api, message.chat.id, &text);

            return;
        }
    };

    messages::edit_message(&bot.api, &progress_msg, "Start sending the post...");

    let result = download::download_contents(&bot.client, result, message.chat.id).await;

    let (root_folder, files) = match result {
        Ok(paths) => paths,
        Err(err) => {
            eprintln!("Error while executing download contents: {}", err);

            messages::delete_message(&bot.api, progress_msg);
            messages::send_message(
                &bot.api,
                message.chat.id,
                "Something went wrong while downloading the post. Please try again later!",
            );

            return;
        }
    };

    if messages::send_medias(&bot.api, message.chat.id, files).is_err() {
        messages::send_message(
            &bot.api,
            message.chat.id,
            "Something went wrong while sending the post. Please try again later!",
        );

        return;
    }

    messages::send_message(&bot.api, message.chat.id, "Finished!");

    if let Err(err) = fs::remove_dir_all(root_folder) {
        eprintln!("Error while deleting folder: {}", err);
    }
}
