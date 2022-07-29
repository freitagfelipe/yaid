use crate::error;
use frankenstein::Message;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io;

#[derive(Deserialize)]
struct ResultPost {
    count: u32,
    urls: Vec<String>,
}

async fn fetch_post<'a>(bot: &crate::Bot, post_url: &str) -> Result<ResultPost, &'a str> {
    let url = env::var("URL").expect("URL must be set!");
    let auth_token = env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set!");

    let response = bot
        .client
        .get(url)
        .query(&[("url", post_url)])
        .header("authorization", auth_token)
        .send()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Error while getting post: {}", err);

            return error!();
        }
    };

    let parsed_response = match response.status() {
        reqwest::StatusCode::OK => match response.json::<ResultPost>().await {
            Ok(parsed) => parsed,
            Err(_) => {
                eprintln!("Error while parsing the object to ResultPost!");

                return error!("a");
            }
        },
        reqwest::StatusCode::NOT_FOUND => {
            return error!("I can't download this post because the post not exists or is from a private account!");
        }
        reqwest::StatusCode::NOT_ACCEPTABLE => {
            return error!("Invaid url! See /help for assistance!");
        }
        status => {
            eprintln!("Response error with status: {}", status);

            return error!();
        }
    };

    Ok(parsed_response)
}

async fn download_post<'a>(
    bot: &crate::Bot,
    url: &str,
    folder_path: &str,
    file_name: usize,
) -> Result<(), Box<dyn Error>> {
    let response = bot.client.get(url).send().await?;

    let file_type = response
        .headers()
        .get("content-type")
        .expect("Response don't have content-type!")
        .to_str()?
        .split("/")
        .last()
        .expect("Last if a None value after split!");

    let mut dest = File::create(format!("{}/{}.{}", folder_path, file_name, file_type))?;
    let content = response.text().await?;

    io::copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}

async fn download_posts<'a>(
    bot: &crate::Bot,
    urls: &Vec<String>,
    chat_id: i64,
) -> Result<(), Box<dyn Error>> {
    let folder_path = format!("./downloads/{}/", chat_id);

    fs::create_dir_all(&folder_path)?;

    for (i, url) in urls.iter().enumerate() {
        download_post(bot, url, &folder_path, i).await?;
    }

    fs::remove_dir(format!("./downloads/{}/", chat_id))?;

    Ok(())
}

pub async fn execute(bot: &crate::Bot, message: Message) {
    let messages = message
        .text
        .as_ref()
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>();
    let post_url = match messages.get(1) {
        Some(url) => *url,
        None => {
            return bot.send_message(
                message.chat.id,
                "Incorrect usage of download. See /help for assistance!",
            )
        }
    };

    let urls = match fetch_post(bot, post_url).await {
        Ok(result) => result.urls,
        Err(text) => return bot.send_message(message.chat.id, text),
    };

    if let Err(err) = download_posts(bot, &urls, message.chat.id).await {
        eprint!("Error while executing download_posts: {}", err);

        bot.send_message(
            message.chat.id,
            "Something went wrong! Please, try again later!",
        );
    }
}
