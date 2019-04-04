extern crate bingmaps;
extern crate locationsharing;
extern crate mammut;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate slog;

mod github;
mod location;
mod mastodon;

pub use github::Github;
pub use location::Location;
pub use mastodon::Mastodon;

#[derive(Debug)]
pub enum Error {
    OtherError(String),
    LocationSharingError(locationsharing::error::Error),
    BingMapsError(bingmaps::Error),
    ReqwestError(reqwest::Error),
    SerdeJSONError(serde_json::Error),
    MammutError(mammut::Error),
}

impl From<locationsharing::error::Error> for Error {
    fn from(source: locationsharing::error::Error) -> Self {
        Error::LocationSharingError(source)
    }
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

impl From<bingmaps::Error> for Error {
    fn from(source: bingmaps::Error) -> Self {
        Error::BingMapsError(source)
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

pub trait Updater {
    fn name(&self) -> &'static str;

    fn new_value(&mut self) -> Result<String, Error>;
}
