use crate::download::{self, ContentType};
use frankenstein::Message;
use std::fs;

pub async fn execute(bot: &crate::Bot, message: Message) {
    let user = message.text.as_ref().unwrap().split(" ").skip(1).last();
    let user = match user {
        Some(name) => name,
        None => {
            bot.send_message(
                message.chat.id,
                "Incorrect usage of download-stories. See /help for assistance!",
            );

            return;
        }
    };

    let progress_msg = bot.send_message(message.chat.id, "⏳Searching user stories...");

    let result = match download::fetch_content(bot, ContentType::Stories(user)).await {
        Ok(result) => result,
        Err(text) => {
            bot.delete_message(progress_msg);
            bot.send_message(message.chat.id, &text);

            return;
        }
    };

    bot.edit_message(&progress_msg, "Start sending the stories...");

    let result = download::download_contents(bot, result, message.chat.id).await;

    let (root_folder, files) = match result {
        Ok(paths) => paths,
        Err(err) => {
            eprintln!("Error while executing download contents: {}", err);

            bot.delete_message(progress_msg);
            bot.send_message(
                message.chat.id,
                "Something went wrong! Please try again later!",
            );

            return;
        }
    };

    bot.send_medias(message.chat.id, files);

    bot.send_message(message.chat.id, "Finished!");

    fs::remove_dir_all(root_folder).unwrap_or_else(|e| {
        eprintln!("Error while deleting folder: {}", e);
    });
}
