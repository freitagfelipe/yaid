use crate::error;
use reqwest::Client;
use serde::Deserialize;
use std::{
    env,
    error::Error,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
pub struct ResultContent {
    urls: Vec<String>,
}

#[derive(Deserialize)]
pub struct ResultError {
    message: String,
}

pub enum ContentType<'a> {
    Post(&'a str),
    Stories(&'a str),
}

pub async fn fetch_content(
    client: &Client,
    content: ContentType<'_>,
) -> Result<ResultContent, String> {
    let base_url = env::var("BASE_URL").unwrap();
    let url = match content {
        ContentType::Post(_) => format!("{base_url}/fetch-post"),
        ContentType::Stories(_) => format!("{base_url}/fetch-stories"),
    };
    let auth_token = env::var("API_TOKEN").unwrap();
    let query = match content {
        ContentType::Post(post) => ("url", post),
        ContentType::Stories(user) => ("user", user),
    };

    let response = client
        .get(url)
        .query(&[query])
        .header("authorization", auth_token)
        .send()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(err) => error!(r: err),
    };

    let response = match response.status() {
        reqwest::StatusCode::OK => match response.json::<ResultContent>().await {
            Ok(parsed) => parsed,
            Err(_) => error!(r: "Can not parse the response to ResultContent!"),
        },
        reqwest::StatusCode::NOT_FOUND | reqwest::StatusCode::FORBIDDEN => {
            let parsed = match response.json::<ResultError>().await {
                Ok(parsed) => parsed,
                Err(_) => error!(r: "Can not parse the response to ResultError!"),
            };

            let reason = format!("{}!", parsed.message);

            error!(e: reason);
        }
        reqwest::StatusCode::NOT_ACCEPTABLE => error!(e: "Invaid url. See /help for assistance!"),
        status => error!(r: format!("Reponse error with status: {status}")),
    };

    Ok(response)
}

async fn download_content(
    client: &Client,
    url: &str,
    folder_path: &Path,
    file_name: usize,
) -> Result<PathBuf, Box<dyn Error>> {
    let response = client.get(url).send().await?;

    let file_type = response
        .headers()
        .get("content-type")
        .expect("Response don not have content-type!")
        .to_str()?
        .split('/')
        .last()
        .expect("Last returned a None value after split!")
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
    client: &Client,
    result: ResultContent,
    chat_id: i64,
) -> Result<(PathBuf, Vec<PathBuf>), Box<dyn Error>> {
    let folder_path = PathBuf::from(format!("./downloads/{chat_id}/"));

    fs::create_dir_all(&folder_path)?;

    let mut files_path = Vec::new();

    for (i, url) in result.urls.iter().enumerate() {
        let fpath = match download_content(client, url, &folder_path, i).await {
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
