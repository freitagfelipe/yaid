use crate::error;
use frankenstein::Message;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::path::PathBuf;

#[derive(Deserialize)]
struct ResultPost {
    urls: Vec<String>,
}

enum FileType {
    JPEG,
    MP4,
}

pub struct DownloadedFile {
    ftype: FileType,
    fpath: PathBuf,
}

impl FileType {
    fn new(ftype: &str) -> Self {
        match ftype {
            "jpeg" => FileType::JPEG,
            "mp4" => FileType::MP4,
            _ => unreachable!(),
        }
    }
}

async fn fetch_post(bot: &crate::Bot, post_url: &str) -> Result<ResultPost, String> {
    let url = env::var("URL").unwrap();
    let auth_token = env::var("AUTH_TOKEN").unwrap();

    let response = bot
        .client
        .get(url)
        .query(&[("url", post_url)])
        .header("authorization", auth_token)
        .send()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(err) => error!(r: err),
    };

    let parsed_response = match response.status() {
        reqwest::StatusCode::OK => match response.json::<ResultPost>().await {
            Ok(parsed) => parsed,
            Err(_) => error!(r: "Can't parse the response to ResultPost!"),
        },
        reqwest::StatusCode::NOT_FOUND => {
            error!(e: "I can't download this post because the post not exists or is from a private account!")
        }
        reqwest::StatusCode::NOT_ACCEPTABLE => error!(e: "Invaid url! See /help for assistance!"),
        status => error!(r: format!("Reponse error with status: {}", status)),
    };

    Ok(parsed_response)
}

async fn download_post(
    bot: &crate::Bot,
    url: &str,
    folder_path: &str,
    file_name: usize,
) -> Result<DownloadedFile, Box<dyn Error>> {
    let response = bot.client.get(url).send().await?;

    let file_type = response
        .headers()
        .get("content-type")
        .expect("Response don't have content-type!")
        .to_str()?
        .split("/")
        .last()
        .expect("Last if a None value after split!")
        .to_string();

    let fpath = PathBuf::from(format!("{}/{}.{}", folder_path, file_name, &file_type));
    let mut dest = File::create(&fpath)?;

    let content = response.bytes().await?;

    io::copy(&mut content.as_ref(), &mut dest)?;

    Ok(DownloadedFile {
        ftype: FileType::new(&file_type),
        fpath,
    })
}

async fn send_posts<'a>(
    bot: &crate::Bot,
    urls: &Vec<String>,
    chat_id: i64,
) -> Result<(), Box<dyn Error>> {
    let folder_path = format!("./downloads/{}/", chat_id);

    fs::create_dir_all(&folder_path)?;

    for (i, url) in urls.iter().enumerate() {
        let file = download_post(bot, url, &folder_path, i).await?;

        match file.ftype {
            FileType::JPEG => bot.send_photo(chat_id, file.fpath),
            FileType::MP4 => bot.send_video(chat_id, file.fpath),
        };
    }

    fs::remove_dir_all(format!("./downloads/{}/", chat_id))?;

    Ok(())
}

pub async fn execute(bot: &crate::Bot, message: Message) {
    let post_url = message.text.as_ref().unwrap().split(" ").skip(1).last();
    let post_url = match post_url {
        Some(url) => url,
        None => {
            bot.send_message(
                message.chat.id,
                "Incorrect usage of download. See /help for assistance!",
            );

            return;
        }
    };

    let progress_msg = bot.send_message(message.chat.id, "⏳Searching your post...");

    let urls = match fetch_post(bot, post_url).await {
        Ok(result) => result.urls,
        Err(text) => {
            bot.delete_message(progress_msg);
            bot.send_message(message.chat.id, &text);

            return;
        }
    };

    bot.edit_message(&progress_msg, "Start sending your post...");

    if let Err(err) = send_posts(bot, &urls, message.chat.id).await {
        eprint!("Error while executing send_posts: {}", err);

        bot.send_message(
            message.chat.id,
            "Something went wrong! Please try again later!",
        );
    }

    bot.send_message(message.chat.id, "Finished!");
}
