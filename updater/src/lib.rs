mod client;
mod cohost;
pub mod github;
pub mod mastodon;

pub use client::Client;
pub use cohost::Cohost;
pub use github::Github;
pub use mastodon::Mastodon;

#[derive(Debug)]
pub enum Error {
    OtherError(String),
    ReqwestError(reqwest::Error),
    SerdeJSONError(serde_json::Error),
    RedisError(redis::RedisError),
    RSSError(rss::Error),
}

impl From<&str> for Error {
    fn from(source: &str) -> Self {
        Error::OtherError(source.into())
    }
}

impl From<String> for Error {
    fn from(source: String) -> Self {
        Error::OtherError(source)
    }
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error::ReqwestError(source)
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::SerdeJSONError(source)
    }
}

impl From<redis::RedisError> for Error {
    fn from(source: redis::RedisError) -> Self {
        Error::RedisError(source)
    }
}

impl From<rss::Error> for Error {
    fn from(source: rss::Error) -> Self {
        Error::RSSError(source)
    }
}

pub trait Updater {
    fn name(&self) -> &'static str;

    fn new_value(&mut self) -> Result<String, Error>;
}
