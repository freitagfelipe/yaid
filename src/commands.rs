mod download;
mod help;
mod start;

use frankenstein::{Api, Message};
use reqwest::Client;

pub fn has_valid_command<'a, 'b>(message: &'a Message) -> Result<&'b str, ()> {
    let text = match message.text.as_ref() {
        Some(text) => text,
        None => return Err(()),
    };

    let command = text.split(" ").collect::<Vec<&str>>()[0];

    match command {
        "/start" => Ok("start"),
        "/download" => Ok("download"),
        "/help" => Ok("help"),
        _ => Err(()),
    }
}

pub async fn execute_command(api: &Api, client: &Client, command: &str, message: Message) {
    match command {
        "start" => start::execute(api, message),
        "download" => download::execute(api, client, message).await,
        "help" => help::execute(api, message),
        _ => unreachable!(),
    }
}
