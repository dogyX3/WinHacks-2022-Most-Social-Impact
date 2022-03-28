use std::error::Error;
use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
enum NewsApiError {
    #[error["Families are fetching articles"]]
    RequestFailed,
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url).call().map_err?(|e | NewsApiError::RequestFailed)?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}