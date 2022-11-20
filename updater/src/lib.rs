extern crate mammut;
extern crate redis;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate slog;

mod client;
pub mod github;
pub mod mastodon;

pub use client::Client;
pub use github::Github;
pub use mastodon::Mastodon;

#[derive(Debug)]
pub enum Error {
    OtherError(String),
    ReqwestError(reqwest::Error),
    SerdeJSONError(serde_json::Error),
    MammutError(mammut::Error),
    RedisError(redis::RedisError),
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

impl From<mammut::Error> for Error {
    fn from(source: mammut::Error) -> Self {
        Error::MammutError(source)
    }
}

impl From<redis::RedisError> for Error {
    fn from(source: redis::RedisError) -> Self {
        Error::RedisError(source)
    }
}

pub trait Updater {
    fn name(&self) -> &'static str;

    fn new_value(&mut self) -> Result<String, Error>;
}
