use crate::error;
use frankenstein::{Api, Message};
use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct ResultPost {
    count: u32,
    urls: Vec<String>,
}

async fn fetch_post<'a, 'b>(client: &Client, post_url: &'a str) -> Result<ResultPost, &'b str> {
    let url = env::var("URL").expect("URL must be set!");
    let auth_token = env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set!");

    let response = client
        .get(url)
        .query(&[("url", post_url)])
        .header("authorization", auth_token)
        .send()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(err) => {
            eprint!("Error while getting post: {}", err);

            return error!();
        }
    };

    let parsed_response = match response.status() {
        reqwest::StatusCode::OK => match response.json::<ResultPost>().await {
            Ok(parsed) => parsed,
            Err(_) => {
                eprint!("Error while parsing the object to ResultPost!");

                return error!();
            }
        },
        reqwest::StatusCode::NOT_FOUND => {
            return error!("I can't download this post because the post not exists or is from a private account!");
        }
        reqwest::StatusCode::NOT_ACCEPTABLE => {
            return error!("Invaid url! See /help for assistance!");
        }
        status => {
            eprint!("Response error with status: {}", status);

            return error!();
        }
    };

    Ok(parsed_response)
}

pub async fn execute(api: &Api, client: &Client, message: Message) {
    let messages = message
        .text
        .as_ref()
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>();
    let post_url = match messages.get(1) {
        Some(url) => *url,
        None => {
            crate::send_message(
                api,
                message.chat.id,
                "Incorrect usage of download. See /help for assistance!",
            );

            return;
        }
    };

    match fetch_post(client, post_url).await {
        Ok(_) => todo!(),
        Err(text) => crate::send_message(api, message.chat.id, text),
    };
}
