use crate::error;
use serde::Deserialize;
use std::{
    env,
    error::Error,
    fs::{self, File},
    io,
    path::PathBuf,
};

#[derive(Deserialize)]
pub struct ResultContent {
    urls: Vec<String>,
}

pub enum ContentType<'a> {
    Post(&'a str),
    Stories(&'a str),
}

pub async fn fetch_content(
    bot: &crate::Bot,
    content: ContentType<'_>,
) -> Result<ResultContent, String> {
    let base_url = env::var("URL").unwrap();
    let url = match content {
        ContentType::Post(_) => format!("{}/fetch-post", base_url),
        ContentType::Stories(_) => format!("{}/fetch-stories", base_url),
    };
    let auth_token = env::var("AUTH_TOKEN").unwrap();
    let query = match content {
        ContentType::Post(post) => ("url", post),
        ContentType::Stories(user) => ("user", user),
    };

    let response = bot
        .client
        .get(url)
        .query(&[query])
        .header("authorization", auth_token)
        .send()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(err) => error!(r: err),
    };

    let parsed_response = match response.status() {
        reqwest::StatusCode::OK => match response.json::<ResultContent>().await {
            Ok(parsed) => parsed,
            Err(_) => error!(r: "Can't parse the response to ResultContent!"),
        },
        reqwest::StatusCode::NOT_FOUND => {
            error!(e: "I can't download this see /help to figure why!")
        }
        reqwest::StatusCode::NOT_ACCEPTABLE => error!(e: "Invaid url. See /help for assistance!"),
        reqwest::StatusCode::FORBIDDEN => {
            error!(e: "I can't download the stories of this profile becaus it's private!")
        }
        status => error!(r: format!("Reponse error with status: {}", status)),
    };

    Ok(parsed_response)
}

async fn download_content(
    bot: &crate::Bot,
    url: &str,
    folder_path: &PathBuf,
    file_name: usize,
) -> Result<PathBuf, Box<dyn Error>> {
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

    let fpath = PathBuf::from(format!(
        "{}/{}.{}",
        folder_path.to_string_lossy(),
        file_name,
        &file_type
    ));
    let mut dest = File::create(&fpath)?;

    let content = response.bytes().await?;

    io::copy(&mut content.as_ref(), &mut dest)?;

    Ok(fpath)
}

pub async fn download_contents(
    bot: &crate::Bot,
    result: ResultContent,
    chat_id: i64,
) -> Result<(PathBuf, Vec<PathBuf>), Box<dyn Error>> {
    let folder_path = PathBuf::from(format!("./downloads/{}/", chat_id));

    fs::create_dir_all(&folder_path)?;

    let mut files_path = Vec::new();

    for (i, url) in result.urls.iter().enumerate() {
        let fpath = match download_content(bot, url, &folder_path, i).await {
            Ok(fpath) => fpath,
            Err(err) => {
                fs::remove_dir_all(&folder_path).unwrap_or_else(|e| {
                    eprintln!("Error while deleting folder: {}", e);
                });

                return Err(err);
            }
        };

        files_path.push(fpath);
    }

    Ok((folder_path, files_path))
}
